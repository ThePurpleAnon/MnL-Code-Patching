#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct VarItemMetadata {
    pub name_id: u16,
    pub description_id: u16,
    pub effect_id: u16,
    pub block_sfx: u8,
    pub sprite_id1: u8,
    pub sprite_id2: u32,
    pub price: u16,
    pub variable: u16,
    pub secondary_variable: u16,
    max_amount: u16,
}

impl VarItemMetadata {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name_id: u16,
        description_id: u16,
        effect_id: u16,
        block_sfx: u8,
        sprite_id1: u8,
        sprite_id2: u32,
        price: u16,
        variable: u16,
        secondary_variable: u16,
        max_amount: u16,
        amount_inverted: bool,
    ) -> Self {
        Self {
            name_id,
            description_id,
            effect_id,
            block_sfx,
            sprite_id1,
            sprite_id2,
            price,
            variable,
            secondary_variable,
            max_amount: max_amount | (u16::from(amount_inverted) << 15),
        }
    }

    pub fn max_amount(&self) -> u16 {
        self.max_amount & 0x7FFF
    }
    pub fn set_max_amount(&mut self, max_amount: u16) {
        self.max_amount = max_amount & 0x7FFF;
    }
    pub fn amount_inverted(&self) -> bool {
        self.max_amount & 0x8000 != 0
    }
    pub fn set_amount_inverted(&mut self, amount_inverted: bool) {
        self.max_amount = self.max_amount() | (u16::from(amount_inverted) << 15);
    }
}
