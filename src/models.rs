use bevy_reflect::Reflect;

#[derive(Reflect)]
pub struct Time {
    pub unix_time: usize,
}

#[derive(Reflect)]
pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: i32,
}

#[derive(Reflect)]
pub struct Geolocation {
    pub lat: isize,
    pub lng: isize,
}
