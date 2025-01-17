use gamercade_audio::{Chain, CHAINS_MAX_COUNT};

use crate::{
    editor_data::{EditorAudioDataEntry, EditorSoundData},
    ui::AudioList,
};

#[derive(Default)]
pub(super) struct ChainList {
    pub(super) selected_chain: usize,
}

impl AudioList<Option<Chain>> for ChainList {
    const MAX_ENTRY_COUNT: usize = CHAINS_MAX_COUNT;
    const NAME: &'static str = "Chain";

    fn target_data_mut(
        data: &mut EditorSoundData,
    ) -> &mut Vec<EditorAudioDataEntry<Option<Chain>>> {
        &mut data.chains
    }

    fn selected_index(&mut self) -> &mut usize {
        &mut self.selected_chain
    }

    fn on_add() -> Option<Chain> {
        Some(Chain::default())
    }

    fn on_clear(&mut self, data: &mut Vec<EditorAudioDataEntry<Option<Chain>>>) {
        data[self.selected_chain].data = Some(Chain::default())
    }
}
