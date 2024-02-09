use crate::section::{
    Colour, Difficulty, Editor, Events, General, HitObject, Metadata, TimingPoint,
};
#[derive(Debug)]
pub struct Beatmap {
    pub(crate) general: General,
    pub(crate) difficulty: Difficulty,
    pub(crate) metadata: Metadata,
    pub(crate) editor: Editor,
    pub(crate) events: Events,
    pub(crate) timing_points: Vec<TimingPoint>,
    pub(crate) colours: Vec<Colour>,
    pub(crate) hit_objects: Vec<HitObject>,
}

impl Beatmap {
    pub fn new(
        general: General,
        editor: Editor,
        metadata: Metadata,
        difficulty: Difficulty,
        events: Events,
        timing_points: Vec<TimingPoint>,
        colours: Vec<Colour>,
        hit_objects: Vec<HitObject>,
    ) -> Self {
        Beatmap {
            general,
            difficulty,
            metadata,
            editor,
            events,
            timing_points,
            colours,
            hit_objects,
        }
    }

    pub fn save(&self, file: &str) {
        std::fs::write(file, self.to_string()).unwrap();
    }

    pub fn get_hit_objects(&self) -> &Vec<HitObject> {
        &self.hit_objects
    }

    pub fn get_timing_points(&self) -> &Vec<TimingPoint> {
        &self.timing_points
    }
}

impl std::fmt::Display for Beatmap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let display_string = format!(
            "{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n[TimingPoints]\n{}\n\n[Colours]\n{}\n\n[HitObjects]\n{}",
            self.general,
            self.editor,
            self.metadata,
            self.difficulty,
            self.events,
            self.timing_points
                .iter()
                .map(|timing_point| timing_point.to_string())
                .collect::<String>(),
            self.colours
                .iter()
                .map(|colour| colour.to_string())
                .collect::<String>(),
            self.hit_objects
                .iter()
                .map(|hit_object| hit_object.to_string())
                .collect::<String>(),
        );

        write!(f, "{}", display_string)
    }
}
