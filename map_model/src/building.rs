// Copyright 2018 Google LLC, licensed under http://www.apache.org/licenses/LICENSE-2.0

use Pt2D;

// TODO reconsider pub usize. maybe outside world shouldnt know.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct BuildingID(pub usize);

#[derive(Debug)]
pub struct Building {
    pub id: BuildingID,
    pub points: Vec<Pt2D>,
    pub osm_tags: Vec<String>,
    pub osm_way_id: i64,
}

impl PartialEq for Building {
    fn eq(&self, other: &Building) -> bool {
        self.id == other.id
    }
}
