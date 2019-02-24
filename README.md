# Ekke_Merge

A general merge trait that can be derived on custom structs. I use it for merging user configuration into default configuration. In order to derive it automatically for a struct, all fields on the struct must implement Merge.

Merge comes with default impls for all primitive types and several std::collections (Vec, HashMap and BTreeMap). Vectors are not merged but replaced, because otherwise we don't have the possibility of removing entries, only adding. Therefor the impl for Vec uses mem::replace as for other types for which merging makes no sense.

Impls for serde_yaml are provided behind a feature ("serdeyaml"). This allows deserializing yaml configuration into your custom type, and make that type mergable.


## TODO
- change should_panic tests to verify the error type
- add other collections from std
- add impls for other formats than yaml
