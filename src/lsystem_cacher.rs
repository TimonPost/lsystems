
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct LSystemKey(&'static str, u8);

pub struct LSystemFactory<T> {
    lsystems_alphabets: HashMap<LSystemKey, Alphabet>,
    lsystems_meshes: HashMap<LSystemKey, Vec<T>>,
}

impl<T> LSystemFactory<T> {
    pub fn new() -> Self {
        Self {
            lsystems_alphabets: HashMap::new(),
            lsystems_meshes: HashMap::new(),
        }
    }

    pub fn generate<L: LSystemDefinition<T>>(
        &mut self,
        definition: &L,
        generation: u8,
    ) -> &Alphabet {
        let key = LSystemKey(definition.name(), generation);

        self.lsystems_alphabets.entry(key).or_insert_with(|| {
            let lsystem = definition.lsystem();
            lsystem.generate(generation)
        })
    }

    pub fn render<L: LSystemDefinition<T>>(
        &mut self,
        definition: &L,
        generation: u8,
        position: Vec3,
        rotation: Vec3,
        scale: f32,
    ) -> Option<&Vec<T>> {
        let key = LSystemKey(definition.name(), generation);
        if let Some(alphabet) = self.lsystems_alphabets.get(&key) {
            let entities = self.lsystems_meshes.entry(key).or_insert_with(|| {
                let lsystem = definition.lsystem();
                lsystem.execute(position, scale, rotation, &alphabet)
            });

            return Some(entities);
        }
        None
    }
}