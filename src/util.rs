use iced::Point;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct TablePoint(#[serde(with = "self")] pub Point);

pub fn serialize<S>(point: &Point, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    (point.x, point.y).serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Point, D::Error> where D: Deserializer<'de> {
    let (x, y) = <(f32, f32)>::deserialize(deserializer)?;
    Ok(Point::new(x, y))
}
