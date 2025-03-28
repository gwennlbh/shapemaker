use super::{probe::Datapoint, remote_probe::RemoteProbe};
use nih_plug::prelude::*;
use rand::Rng;
use std::sync::Arc;

pub struct ShapemakerVST {
    params: Arc<ShapemakerVSTParams>,
    probe: RemoteProbe,
}

#[derive(Params)]
struct ShapemakerVSTParams {
    /// Used to send automation data to Shapemaker
    #[id = "param1"]
    pub param1: FloatParam,
    #[id = "param2"]
    pub param2: FloatParam,
    #[id = "param3"]
    pub param3: FloatParam,
    #[id = "param4"]
    pub param4: FloatParam,
    #[id = "param5"]
    pub param5: FloatParam,
    #[id = "param6"]
    pub param6: FloatParam,
    #[id = "param7"]
    pub param7: FloatParam,
    #[id = "param8"]
    pub param8: FloatParam,
    #[id = "param9"]
    pub param9: FloatParam,
}

impl Default for ShapemakerVST {
    fn default() -> Self {
        let probe_id = rand::thread_rng().gen_range(1..=u32::MAX);
        Self {
            params: Arc::new(ShapemakerVSTParams::default()),
            probe: RemoteProbe::new(probe_id),
        }
    }
}

impl Default for ShapemakerVSTParams {
    fn default() -> Self {
        let paramdef = |id: usize| {
            FloatParam::new(
                format!("Param #{}", id),
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.1 },
            )
        };
        Self {
            param1: paramdef(1),
            param2: paramdef(2),
            param3: paramdef(3),
            param4: paramdef(4),
            param5: paramdef(5),
            param6: paramdef(6),
            param7: paramdef(7),
            param8: paramdef(8),
            param9: paramdef(9),
        }
    }
}

impl Plugin for ShapemakerVST {
    const NAME: &'static str = "Shapemaker VST b3";
    const VENDOR: &'static str = "Gwenn Le Bihan";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "gwenn.lebihan7@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // The first audio IO layout is used as the default. The other layouts may be selected either
    // explicitly or automatically by the host or the user depending on the plugin API/backend.
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        // Individual ports and the layout as a whole can be named here. By default these names
        // are generated as needed. This layout will be called 'Stereo', while a layout with
        // only one input and output channel would be called 'Mono'.
        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::MidiCCs;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // If the plugin can send or receive SysEx messages, it can define a type to wrap around those
    // messages here. The type implements the `SysExMessage` trait, which allows conversion to and
    // from plain byte buffers.
    type SysExMessage = ();
    // More advanced plugins can use this to run expensive background tasks. See the field's
    // documentation for more information. `()` means that the plugin does not have any background
    // tasks.
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        let _ = self.probe.register();
        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
    }

    fn deactivate(&mut self) {
        // probe should be removed from beacon thanks to the Drop impl
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let ts = RemoteProbe::timestamp();
        // self.probe.say(format!("{} sending data", ts));
        self.probe.store_automation(ts, 1, &self.params.param1);
        self.probe.store_automation(ts, 2, &self.params.param2);
        self.probe.store_automation(ts, 3, &self.params.param3);
        self.probe.store_automation(ts, 4, &self.params.param4);
        self.probe.store_automation(ts, 4, &self.params.param4);
        self.probe.store_automation(ts, 5, &self.params.param5);
        self.probe.store_automation(ts, 6, &self.params.param6);
        self.probe.store_automation(ts, 7, &self.params.param7);
        self.probe.store_automation(ts, 8, &self.params.param8);
        self.probe.store_automation(ts, 9, &self.params.param9);
        // self.probe.say(format!("{} sent automation", ts));

        // self.probe.store_audio(
        //     ts,
        //     buffer
        //         .iter_samples()
        //         .flatten()
        //         .map(|f| *f)
        //         .collect::<Vec<f32>>(),
        // );
        // self.probe.say(format!("{} sent audio", ts));

        ProcessStatus::Normal
    }
}

impl ClapPlugin for ShapemakerVST {
    const CLAP_ID: &'static str = "works.gwen.shapemakervst";
    const CLAP_DESCRIPTION: Option<&'static str> =
        Some("A VST plugin for Shapemaker, an experimental audiovisual SVG-based rendering engine");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] =
        &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for ShapemakerVST {
    const VST3_CLASS_ID: [u8; 16] = *b"gwennlbhshapemak";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}
