use super::Screen;
use crate::{Result as CResult, Wrapper};
use async_trait::async_trait;
use mergui::MFont;
use quicksilver::{
    geom::Vector,
    graphics::{Color, VectorFont},
};

pub(crate) struct BattleOver {
    font: MFont,
}

impl BattleOver {
    pub(crate) async fn new(wrapper: &mut Wrapper) -> CResult<Self> {
        let ttf = VectorFont::load("font.ttf").await?;
        let font = MFont::from_font(&ttf, &wrapper.gfx, 30.0)?;
        Ok(BattleOver { font })
    }
}

#[async_trait(?Send)]
impl Screen for BattleOver {
    async fn draw(&mut self, wrapper: &mut Wrapper) -> CResult<()> {
        wrapper.gfx.clear(Color::WHITE);
        self.font.draw(
            &mut wrapper.gfx,
            "Battle over",
            Color::BLACK,
            Vector::new(558., 324.),
        )?;
        Ok(())
    }
    async fn update(&mut self, _: &mut Wrapper) -> CResult<Option<Box<dyn Screen>>> {
        Ok(None)
    }
}
