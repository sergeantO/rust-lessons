/// провайдер для генерации id
pub trait IDProvider {
    /// получить новый id
    fn provide(&self) -> String;
}

/// боевая реализация провайдера для генерации id
pub struct NanoIdProvider;

impl IDProvider for NanoIdProvider {
    fn provide(&self) -> String {
        nanoid::nanoid!(12)
    }
}

/// Реализация IDProvider для тестирования
pub struct FakeIDProvider {
    id: String,
}

impl FakeIDProvider {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id
    }
}

impl IDProvider for FakeIDProvider {
    fn provide(&self) -> String {
        self.id.clone()
    }
}
