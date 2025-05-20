use crate::lego1::realtime::roi::lod_object::LODObject;

pub type LodListBase = Vec<Box<dyn LODObject>>;

pub type LODList = LodListBase;