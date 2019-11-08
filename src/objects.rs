use crate::*;

impl ZMachine {
    /// Returns the base of the object table (i.e. at the start of the property defaults header).
    pub fn object_table_base(&self) -> ByteAddress {
        self.word(ByteAddress::OBJECT_TABLE_LOCATION).into()
    }
    /// Returns the number of properties an object can have in this story version.
    pub fn object_property_count(&self) -> usize {
        if self.version() > Version::V3 {
            63
        } else {
            31
        }
    }
    /// Returns the number of attributes an object has in this story version.
    pub fn object_attribute_count(&self) -> usize {
        if self.version() > Version::V3 {
            48
        } else {
            32
        }
    }
    /// Returns the maximum number of objects in this story version.
    pub fn max_objects_for_version(&self) -> usize {
        if self.version() > Version::V3 {
            65535
        } else {
            255
        }
    }
    fn object_entry_size(&self) -> usize {
        if self.version() > Version::V3 {
            14
        } else {
            9
        }
    }
    /// Returns the number of objects in play.
    ///
    /// # Note
    ///
    /// This is an educated guess based on the locations of certain structures. A story could
    /// lay out memory such that this fails, but no known stories do.
    pub fn objects_count(&self) -> usize {
        let object_1 = self.object_unchecked(1);
        (object_1.property_table_location() - object_1.start) / self.object_entry_size()
    }
    /// Returns the default value for a property. Panics if `property_id` is out of bounds
    /// ([`object_property_count`](ZMachine::object_property_count))
    pub fn default_property(&self, property_id: usize) -> &[u8] {
        let ct = self.object_property_count();
        assert!(
            property_id < ct,
            "Property ID out of bounds (was {}, max {})",
            property_id,
            ct
        );
        let idx = self.object_table_base() + property_id * 2;
        &self[idx..=(idx + 1)]
    }
    fn object_table_objects_start(&self) -> ByteAddress {
        self.object_table_base() + self.object_property_count() * 2
    }
    /// Returns an object with a particular ID. Panics if `id` is out of bounds
    /// (1..[`objects_count`](ZMachine::objects_count))
    pub fn object(&self, id: usize) -> Object {
        assert!(
            id != 0 && id < self.objects_count(),
            "Object ID out of bounds (was {}, requires 1..{})",
            id,
            self.objects_count()
        );
        self.object_unchecked(id)
    }
    /// Returns an object with a particular ID. Does not do any bounds checking. Do not use unless
    /// you are sure [`objects_count`](ZMachine::objects_count) is wrong.
    pub fn object_unchecked(&self, id: usize) -> Object {
        let start = self.object_table_objects_start();
        let sz = self.object_entry_size();
        let addr = start + sz * (id - 1);
        Object {
            start: addr,
            machine: self,
        }
    }
}

/// Represents a game object.
pub struct Object<'a> {
    start: ByteAddress,
    machine: &'a ZMachine,
}

impl<'a> Object<'a> {
    /// Returns a particular attribute from this object. Panics if `attribute_id` is out of bounds
    /// ([`ZMachine::object_attribute_count`])
    pub fn attribute(&self, attribute_id: usize) -> bool {
        assert!(
            attribute_id < self.machine.object_attribute_count(),
            "Attribute ID out of range (was {}, max {})",
            attribute_id,
            self.machine.object_attribute_count() - 1
        );
        self.machine[BitAddress::from(self.start) + attribute_id]
    }
    /// Returns the object ID of this object's parent, or `None` if this object has no parent.
    pub fn parent_id(&self) -> Option<usize> {
        let id = if self.machine.version() > Version::V3 {
            self.machine.word(self.start + 6) as usize
        } else {
            self.machine[self.start + 4] as usize
        };
        if id == 0 {
            None
        } else {
            Some(id)
        }
    }
    /// Returns this object's parent, or `None` if this object has no parent.
    pub fn parent(&self) -> Option<Object<'a>> {
        self.parent_id().map(|x| self.machine.object_unchecked(x))
    }
    /// Returns the object ID of this object's sibling, or `None` if this object has no sibling.
    pub fn sibling_id(&self) -> Option<usize> {
        let id = if self.machine.version() > Version::V3 {
            self.machine.word(self.start + 8) as usize
        } else {
            self.machine[self.start + 5] as usize
        };
        if id == 0 {
            None
        } else {
            Some(id)
        }
    }
    /// Returns this object's sibling, or `None` if this object has no sibling.
    pub fn sibling(&self) -> Option<Object<'a>> {
        self.sibling_id().map(|x| self.machine.object_unchecked(x))
    }
    /// Returns the object ID of this object's child, or `None` if this object has no child.
    pub fn child_id(&self) -> Option<usize> {
        let id = if self.machine.version() > Version::V3 {
            self.machine.word(self.start + 10) as usize
        } else {
            self.machine[self.start + 6] as usize
        };
        if id == 0 {
            None
        } else {
            Some(id)
        }
    }
    /// Returns this object's child, or `None` if this object has no child.
    pub fn child(&self) -> Option<Object<'a>> {
        self.child_id().map(|x| self.machine.object_unchecked(x))
    }
    /// Returns the address of this object's property table.
    pub fn property_table_location(&self) -> ByteAddress {
        if self.machine.version() > Version::V3 {
            self.machine.word(self.start + 12).into()
        } else {
            self.machine.word(self.start + 7).into()
        }
    }
    /// Returns the short name of this object.
    pub fn read_name(&self) -> String {
        let mut string = String::new();
        self.copy_name(&mut string);
        string
    }
    fn properties_start(&self) -> ByteAddress {
        let tbl = self.property_table_location();
        tbl + self.machine[tbl] as usize + 1
    }
    /// Copies the short name of this object into the provided buffer.
    pub fn copy_name(&self, string: &mut String) {
        let name_addr = self.property_table_location() + 1;
        self.machine.copy_zstring(name_addr, string);
    }
    /// Returns the number of properties on this object.
    pub fn property_count(&self) -> usize {
        let start = self.properties_start();
        let sz_byte1 = self.machine[start] as usize;
        sz_byte1 & 0b00111111
    }
    // untested
    /// Returns the value of a property at a particular ID, or `None` if the property is unset.
    /// Panics if `property_id` is out of bounds (`1..=`[`ZMachine::objects_property_count`])
    pub fn property_value(&self, property_id: usize) -> Option<&'a [u8]> {
        assert!(
            property_id <= self.machine.object_property_count(),
            "Property ID {} out of bounds ({})",
            property_id,
            self.machine.object_property_count()
        );
        if property_id >= self.property_count() {
            return None;
        }
        let mut addr = self.properties_start();
        if self.machine.version() > Version::V3 {
            loop {
                let sz_byte1 = self.machine[addr];
                if sz_byte1 == 0 {
                    break None;
                }
                addr += 1;
                let idx = sz_byte1 & 0b00111111; // sz_byte1 % 32
                let sz = if sz_byte1 & 0b10000000 == 0b10000000 {
                    let sz_byte2 = self.machine[addr];
                    addr += 1;
                    if sz_byte2 == 0 {
                        64
                    } else {
                        sz_byte2 & 0b00111111 // sz_byte2 % 32
                    }
                } else if sz_byte1 & 0b01000000 == 0 {
                    2
                } else {
                    1
                } as usize;
                if idx as usize == property_id {
                    break Some(&self.machine[addr..(addr + sz)]);
                } else {
                    addr += sz;
                }
            }
        } else {
            loop {
                let sz_byte = self.machine[addr];
                if sz_byte == 0 {
                    break None;
                }
                addr += 1;
                let idx = sz_byte & 0b00111111; // sz_byte % 32
                let sz = ((sz_byte >> 6) + 1) as usize; // sz_byte / 32 + 1
                if idx as usize == property_id {
                    break Some(&self.machine[addr..(addr + sz)]);
                } else {
                    addr += sz;
                }
            }
        }
    }
    /// Returns the value of a property at a particular ID, or the equivalent
    /// [`ZMachine::default_property`] if the property is unset. Panics if `property_id` is out of
    /// bounds (`1..=`[`ZMachine::objects_property_count`])
    pub fn property_value_or_default(&self, property_id: usize) -> &'a [u8] {
        if let Some(slice) = self.property_value(property_id) {
            slice
        } else {
            self.machine.default_property(property_id)
        }
    }
}
