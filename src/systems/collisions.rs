use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let zero_pos = Point::zero();

    let player_pos = <&Point>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .last()
        .unwrap_or(&zero_pos);

    <(Entity, &Point)>::query()
        .filter(component::<Enemy>())
        .iter(ecs)
        .filter(|(_, pos)| **pos == *player_pos)
        .for_each(|(entity, _)| commands.remove(*entity));
}
