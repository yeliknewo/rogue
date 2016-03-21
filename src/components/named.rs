use logic::{EntityData, World, WorldErr, Id};

pub struct Named {
    name: &'static str,
}

impl Named {
    pub fn new<T: EntityData<T>>(name: &'static str, id: Id, world: &mut World<T>) -> Result<Named, NamedErr> {
        match world.register_name(id, name) {
            Ok(_) => {
                Ok(
                    Named {
                        name: name,
                    }
                )
            },
            Err(err) => Err(NamedErr::New(err)),
        }
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }
}

#[derive(Debug)]
pub enum NamedErr {
    New(WorldErr),
}
