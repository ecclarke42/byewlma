///! Automatically generated code. Do not manually edit
use strum::IntoStaticStr;

impl IconKind {
    pub fn name(&self) -> &'static str {
        let kind = *self;
        if let IconKind::Custom(name) = kind {
            name
        } else {
            kind.into()
        }
    }
}

/// A Fontawesome free/solid icon.
///
/// There are a lot (>1000) possible icons here,
#[derive(Debug, IntoStaticStr, Clone, Copy, PartialEq, Eq)]
pub enum IconKind {
    #[strum(disabled)]
    Custom(&'static str),

    /// ad: \uf641
    ///
    /// https://fontawesome.com/v5.15/icons/ad?style=solid
    #[strum(serialize = "fa-ad")]
    Ad,

    /// address-book: \uf2b9
    ///
    /// https://fontawesome.com/v5.15/icons/address-book?style=solid
    #[strum(serialize = "fa-address-book")]
    AddressBook,

    /// address-card: \uf2bb
    ///
    /// https://fontawesome.com/v5.15/icons/address-card?style=solid
    #[strum(serialize = "fa-address-card")]
    AddressCard,

    /// adjust: \uf042
    ///
    /// https://fontawesome.com/v5.15/icons/adjust?style=solid
    #[strum(serialize = "fa-adjust")]
    Adjust,

    /// air-freshener: \uf5d0
    ///
    /// https://fontawesome.com/v5.15/icons/air-freshener?style=solid
    #[strum(serialize = "fa-air-freshener")]
    AirFreshener,

    /// align-center: \uf037
    ///
    /// https://fontawesome.com/v5.15/icons/align-center?style=solid
    #[strum(serialize = "fa-align-center")]
    AlignCenter,

    /// align-justify: \uf039
    ///
    /// https://fontawesome.com/v5.15/icons/align-justify?style=solid
    #[strum(serialize = "fa-align-justify")]
    AlignJustify,

    /// align-left: \uf036
    ///
    /// https://fontawesome.com/v5.15/icons/align-left?style=solid
    #[strum(serialize = "fa-align-left")]
    AlignLeft,

    /// align-right: \uf038
    ///
    /// https://fontawesome.com/v5.15/icons/align-right?style=solid
    #[strum(serialize = "fa-align-right")]
    AlignRight,

    /// allergies: \uf461
    ///
    /// https://fontawesome.com/v5.15/icons/allergies?style=solid
    #[strum(serialize = "fa-allergies")]
    Allergies,

    /// ambulance: \uf0f9
    ///
    /// https://fontawesome.com/v5.15/icons/ambulance?style=solid
    #[strum(serialize = "fa-ambulance")]
    Ambulance,

    /// american-sign-language-interpreting: \uf2a3
    ///
    /// https://fontawesome.com/v5.15/icons/american-sign-language-interpreting?style=solid
    #[strum(serialize = "fa-american-sign-language-interpreting")]
    AmericanSignLanguageInterpreting,

    /// anchor: \uf13d
    ///
    /// https://fontawesome.com/v5.15/icons/anchor?style=solid
    #[strum(serialize = "fa-anchor")]
    Anchor,

    /// angle-double-down: \uf103
    ///
    /// https://fontawesome.com/v5.15/icons/angle-double-down?style=solid
    #[strum(serialize = "fa-angle-double-down")]
    AngleDoubleDown,

    /// angle-double-left: \uf100
    ///
    /// https://fontawesome.com/v5.15/icons/angle-double-left?style=solid
    #[strum(serialize = "fa-angle-double-left")]
    AngleDoubleLeft,

    /// angle-double-right: \uf101
    ///
    /// https://fontawesome.com/v5.15/icons/angle-double-right?style=solid
    #[strum(serialize = "fa-angle-double-right")]
    AngleDoubleRight,

    /// angle-double-up: \uf102
    ///
    /// https://fontawesome.com/v5.15/icons/angle-double-up?style=solid
    #[strum(serialize = "fa-angle-double-up")]
    AngleDoubleUp,

    /// angle-down: \uf107
    ///
    /// https://fontawesome.com/v5.15/icons/angle-down?style=solid
    #[strum(serialize = "fa-angle-down")]
    AngleDown,

    /// angle-left: \uf104
    ///
    /// https://fontawesome.com/v5.15/icons/angle-left?style=solid
    #[strum(serialize = "fa-angle-left")]
    AngleLeft,

    /// angle-right: \uf105
    ///
    /// https://fontawesome.com/v5.15/icons/angle-right?style=solid
    #[strum(serialize = "fa-angle-right")]
    AngleRight,

    /// angle-up: \uf106
    ///
    /// https://fontawesome.com/v5.15/icons/angle-up?style=solid
    #[strum(serialize = "fa-angle-up")]
    AngleUp,

    /// angry: \uf556
    ///
    /// https://fontawesome.com/v5.15/icons/angry?style=solid
    #[strum(serialize = "fa-angry")]
    Angry,

    /// ankh: \uf644
    ///
    /// https://fontawesome.com/v5.15/icons/ankh?style=solid
    #[strum(serialize = "fa-ankh")]
    Ankh,

    /// apple-alt: \uf5d1
    ///
    /// https://fontawesome.com/v5.15/icons/apple-alt?style=solid
    #[strum(serialize = "fa-apple-alt")]
    AppleAlt,

    /// archive: \uf187
    ///
    /// https://fontawesome.com/v5.15/icons/archive?style=solid
    #[strum(serialize = "fa-archive")]
    Archive,

    /// archway: \uf557
    ///
    /// https://fontawesome.com/v5.15/icons/archway?style=solid
    #[strum(serialize = "fa-archway")]
    Archway,

    /// arrow-alt-circle-down: \uf358
    ///
    /// https://fontawesome.com/v5.15/icons/arrow-alt-circle-down?style=solid
    #[strum(serialize = "fa-arrow-alt-circle-down")]
    ArrowAltCircleDown,

    /// arrow-alt-circle-left: \uf359
    ///
    /// https://fontawesome.com/v5.15/icons/arrow-alt-circle-left?style=solid
    #[strum(serialize = "fa-arrow-alt-circle-left")]
    ArrowAltCircleLeft,

    /// arrow-alt-circle-right: \uf35a
    ///
    /// https://fontawesome.com/v5.15/icons/arrow-alt-circle-right?style=solid
    #[strum(serialize = "fa-arrow-alt-circle-right")]
    ArrowAltCircleRight,

    /// arrow-alt-circle-up: \uf35b
    ///
    /// https://fontawesome.com/v5.15/icons/arrow-alt-circle-up?style=solid
    #[strum(serialize = "fa-arrow-alt-circle-up")]
    ArrowAltCircleUp,

    /// arrow-circle-down: \uf0ab
    ///
    /// https://fontawesome.com/v5.15/icons/arrow-circle-down?style=solid
    #[strum(serialize = "fa-arrow-circle-down")]
    ArrowCircleDown,

    /// arrow-circle-left: \uf0a8
    ///
    /// https://fontawesome.com/v5.15/icons/arrow-circle-left?style=solid
    #[strum(serialize = "fa-arrow-circle-left")]
    ArrowCircleLeft,

    /// arrow-circle-right: \uf0a9
    ///
    /// https://fontawesome.com/v5.15/icons/arrow-circle-right?style=solid
    #[strum(serialize = "fa-arrow-circle-right")]
    ArrowCircleRight,

    /// arrow-circle-up: \uf0aa
    ///
    /// https://fontawesome.com/v5.15/icons/arrow-circle-up?style=solid
    #[strum(serialize = "fa-arrow-circle-up")]
    ArrowCircleUp,

    /// arrow-down: \uf063
    ///
    /// https://fontawesome.com/v5.15/icons/arrow-down?style=solid
    #[strum(serialize = "fa-arrow-down")]
    ArrowDown,

    /// arrow-left: \uf060
    ///
    /// https://fontawesome.com/v5.15/icons/arrow-left?style=solid
    #[strum(serialize = "fa-arrow-left")]
    ArrowLeft,

    /// arrow-right: \uf061
    ///
    /// https://fontawesome.com/v5.15/icons/arrow-right?style=solid
    #[strum(serialize = "fa-arrow-right")]
    ArrowRight,

    /// arrow-up: \uf062
    ///
    /// https://fontawesome.com/v5.15/icons/arrow-up?style=solid
    #[strum(serialize = "fa-arrow-up")]
    ArrowUp,

    /// arrows-alt: \uf0b2
    ///
    /// https://fontawesome.com/v5.15/icons/arrows-alt?style=solid
    #[strum(serialize = "fa-arrows-alt")]
    ArrowsAlt,

    /// arrows-alt-h: \uf337
    ///
    /// https://fontawesome.com/v5.15/icons/arrows-alt-h?style=solid
    #[strum(serialize = "fa-arrows-alt-h")]
    ArrowsAltH,

    /// arrows-alt-v: \uf338
    ///
    /// https://fontawesome.com/v5.15/icons/arrows-alt-v?style=solid
    #[strum(serialize = "fa-arrows-alt-v")]
    ArrowsAltV,

    /// assistive-listening-systems: \uf2a2
    ///
    /// https://fontawesome.com/v5.15/icons/assistive-listening-systems?style=solid
    #[strum(serialize = "fa-assistive-listening-systems")]
    AssistiveListeningSystems,

    /// asterisk: \uf069
    ///
    /// https://fontawesome.com/v5.15/icons/asterisk?style=solid
    #[strum(serialize = "fa-asterisk")]
    Asterisk,

    /// at: \uf1fa
    ///
    /// https://fontawesome.com/v5.15/icons/at?style=solid
    #[strum(serialize = "fa-at")]
    At,

    /// atlas: \uf558
    ///
    /// https://fontawesome.com/v5.15/icons/atlas?style=solid
    #[strum(serialize = "fa-atlas")]
    Atlas,

    /// atom: \uf5d2
    ///
    /// https://fontawesome.com/v5.15/icons/atom?style=solid
    #[strum(serialize = "fa-atom")]
    Atom,

    /// audio-description: \uf29e
    ///
    /// https://fontawesome.com/v5.15/icons/audio-description?style=solid
    #[strum(serialize = "fa-audio-description")]
    AudioDescription,

    /// award: \uf559
    ///
    /// https://fontawesome.com/v5.15/icons/award?style=solid
    #[strum(serialize = "fa-award")]
    Award,

    /// baby: \uf77c
    ///
    /// https://fontawesome.com/v5.15/icons/baby?style=solid
    #[strum(serialize = "fa-baby")]
    Baby,

    /// baby-carriage: \uf77d
    ///
    /// https://fontawesome.com/v5.15/icons/baby-carriage?style=solid
    #[strum(serialize = "fa-baby-carriage")]
    BabyCarriage,

    /// backspace: \uf55a
    ///
    /// https://fontawesome.com/v5.15/icons/backspace?style=solid
    #[strum(serialize = "fa-backspace")]
    Backspace,

    /// backward: \uf04a
    ///
    /// https://fontawesome.com/v5.15/icons/backward?style=solid
    #[strum(serialize = "fa-backward")]
    Backward,

    /// bacon: \uf7e5
    ///
    /// https://fontawesome.com/v5.15/icons/bacon?style=solid
    #[strum(serialize = "fa-bacon")]
    Bacon,

    /// bacteria: \ue059
    ///
    /// https://fontawesome.com/v5.15/icons/bacteria?style=solid
    #[strum(serialize = "fa-bacteria")]
    Bacteria,

    /// bacterium: \ue05a
    ///
    /// https://fontawesome.com/v5.15/icons/bacterium?style=solid
    #[strum(serialize = "fa-bacterium")]
    Bacterium,

    /// bahai: \uf666
    ///
    /// https://fontawesome.com/v5.15/icons/bahai?style=solid
    #[strum(serialize = "fa-bahai")]
    Bahai,

    /// balance-scale: \uf24e
    ///
    /// https://fontawesome.com/v5.15/icons/balance-scale?style=solid
    #[strum(serialize = "fa-balance-scale")]
    BalanceScale,

    /// balance-scale-left: \uf515
    ///
    /// https://fontawesome.com/v5.15/icons/balance-scale-left?style=solid
    #[strum(serialize = "fa-balance-scale-left")]
    BalanceScaleLeft,

    /// balance-scale-right: \uf516
    ///
    /// https://fontawesome.com/v5.15/icons/balance-scale-right?style=solid
    #[strum(serialize = "fa-balance-scale-right")]
    BalanceScaleRight,

    /// ban: \uf05e
    ///
    /// https://fontawesome.com/v5.15/icons/ban?style=solid
    #[strum(serialize = "fa-ban")]
    Ban,

    /// band-aid: \uf462
    ///
    /// https://fontawesome.com/v5.15/icons/band-aid?style=solid
    #[strum(serialize = "fa-band-aid")]
    BandAid,

    /// barcode: \uf02a
    ///
    /// https://fontawesome.com/v5.15/icons/barcode?style=solid
    #[strum(serialize = "fa-barcode")]
    Barcode,

    /// bars: \uf0c9
    ///
    /// https://fontawesome.com/v5.15/icons/bars?style=solid
    #[strum(serialize = "fa-bars")]
    Bars,

    /// baseball-ball: \uf433
    ///
    /// https://fontawesome.com/v5.15/icons/baseball-ball?style=solid
    #[strum(serialize = "fa-baseball-ball")]
    BaseballBall,

    /// basketball-ball: \uf434
    ///
    /// https://fontawesome.com/v5.15/icons/basketball-ball?style=solid
    #[strum(serialize = "fa-basketball-ball")]
    BasketballBall,

    /// bath: \uf2cd
    ///
    /// https://fontawesome.com/v5.15/icons/bath?style=solid
    #[strum(serialize = "fa-bath")]
    Bath,

    /// battery-empty: \uf244
    ///
    /// https://fontawesome.com/v5.15/icons/battery-empty?style=solid
    #[strum(serialize = "fa-battery-empty")]
    BatteryEmpty,

    /// battery-full: \uf240
    ///
    /// https://fontawesome.com/v5.15/icons/battery-full?style=solid
    #[strum(serialize = "fa-battery-full")]
    BatteryFull,

    /// battery-half: \uf242
    ///
    /// https://fontawesome.com/v5.15/icons/battery-half?style=solid
    #[strum(serialize = "fa-battery-half")]
    BatteryHalf,

    /// battery-quarter: \uf243
    ///
    /// https://fontawesome.com/v5.15/icons/battery-quarter?style=solid
    #[strum(serialize = "fa-battery-quarter")]
    BatteryQuarter,

    /// battery-three-quarters: \uf241
    ///
    /// https://fontawesome.com/v5.15/icons/battery-three-quarters?style=solid
    #[strum(serialize = "fa-battery-three-quarters")]
    BatteryThreeQuarters,

    /// bed: \uf236
    ///
    /// https://fontawesome.com/v5.15/icons/bed?style=solid
    #[strum(serialize = "fa-bed")]
    Bed,

    /// beer: \uf0fc
    ///
    /// https://fontawesome.com/v5.15/icons/beer?style=solid
    #[strum(serialize = "fa-beer")]
    Beer,

    /// bell: \uf0f3
    ///
    /// https://fontawesome.com/v5.15/icons/bell?style=solid
    #[strum(serialize = "fa-bell")]
    Bell,

    /// bell-slash: \uf1f6
    ///
    /// https://fontawesome.com/v5.15/icons/bell-slash?style=solid
    #[strum(serialize = "fa-bell-slash")]
    BellSlash,

    /// bezier-curve: \uf55b
    ///
    /// https://fontawesome.com/v5.15/icons/bezier-curve?style=solid
    #[strum(serialize = "fa-bezier-curve")]
    BezierCurve,

    /// bible: \uf647
    ///
    /// https://fontawesome.com/v5.15/icons/bible?style=solid
    #[strum(serialize = "fa-bible")]
    Bible,

    /// bicycle: \uf206
    ///
    /// https://fontawesome.com/v5.15/icons/bicycle?style=solid
    #[strum(serialize = "fa-bicycle")]
    Bicycle,

    /// biking: \uf84a
    ///
    /// https://fontawesome.com/v5.15/icons/biking?style=solid
    #[strum(serialize = "fa-biking")]
    Biking,

    /// binoculars: \uf1e5
    ///
    /// https://fontawesome.com/v5.15/icons/binoculars?style=solid
    #[strum(serialize = "fa-binoculars")]
    Binoculars,

    /// biohazard: \uf780
    ///
    /// https://fontawesome.com/v5.15/icons/biohazard?style=solid
    #[strum(serialize = "fa-biohazard")]
    Biohazard,

    /// birthday-cake: \uf1fd
    ///
    /// https://fontawesome.com/v5.15/icons/birthday-cake?style=solid
    #[strum(serialize = "fa-birthday-cake")]
    BirthdayCake,

    /// blender: \uf517
    ///
    /// https://fontawesome.com/v5.15/icons/blender?style=solid
    #[strum(serialize = "fa-blender")]
    Blender,

    /// blender-phone: \uf6b6
    ///
    /// https://fontawesome.com/v5.15/icons/blender-phone?style=solid
    #[strum(serialize = "fa-blender-phone")]
    BlenderPhone,

    /// blind: \uf29d
    ///
    /// https://fontawesome.com/v5.15/icons/blind?style=solid
    #[strum(serialize = "fa-blind")]
    Blind,

    /// blog: \uf781
    ///
    /// https://fontawesome.com/v5.15/icons/blog?style=solid
    #[strum(serialize = "fa-blog")]
    Blog,

    /// bold: \uf032
    ///
    /// https://fontawesome.com/v5.15/icons/bold?style=solid
    #[strum(serialize = "fa-bold")]
    Bold,

    /// bolt: \uf0e7
    ///
    /// https://fontawesome.com/v5.15/icons/bolt?style=solid
    #[strum(serialize = "fa-bolt")]
    Bolt,

    /// bomb: \uf1e2
    ///
    /// https://fontawesome.com/v5.15/icons/bomb?style=solid
    #[strum(serialize = "fa-bomb")]
    Bomb,

    /// bone: \uf5d7
    ///
    /// https://fontawesome.com/v5.15/icons/bone?style=solid
    #[strum(serialize = "fa-bone")]
    Bone,

    /// bong: \uf55c
    ///
    /// https://fontawesome.com/v5.15/icons/bong?style=solid
    #[strum(serialize = "fa-bong")]
    Bong,

    /// book: \uf02d
    ///
    /// https://fontawesome.com/v5.15/icons/book?style=solid
    #[strum(serialize = "fa-book")]
    Book,

    /// book-dead: \uf6b7
    ///
    /// https://fontawesome.com/v5.15/icons/book-dead?style=solid
    #[strum(serialize = "fa-book-dead")]
    BookDead,

    /// book-medical: \uf7e6
    ///
    /// https://fontawesome.com/v5.15/icons/book-medical?style=solid
    #[strum(serialize = "fa-book-medical")]
    BookMedical,

    /// book-open: \uf518
    ///
    /// https://fontawesome.com/v5.15/icons/book-open?style=solid
    #[strum(serialize = "fa-book-open")]
    BookOpen,

    /// book-reader: \uf5da
    ///
    /// https://fontawesome.com/v5.15/icons/book-reader?style=solid
    #[strum(serialize = "fa-book-reader")]
    BookReader,

    /// bookmark: \uf02e
    ///
    /// https://fontawesome.com/v5.15/icons/bookmark?style=solid
    #[strum(serialize = "fa-bookmark")]
    Bookmark,

    /// border-all: \uf84c
    ///
    /// https://fontawesome.com/v5.15/icons/border-all?style=solid
    #[strum(serialize = "fa-border-all")]
    BorderAll,

    /// border-none: \uf850
    ///
    /// https://fontawesome.com/v5.15/icons/border-none?style=solid
    #[strum(serialize = "fa-border-none")]
    BorderNone,

    /// border-style: \uf853
    ///
    /// https://fontawesome.com/v5.15/icons/border-style?style=solid
    #[strum(serialize = "fa-border-style")]
    BorderStyle,

    /// bowling-ball: \uf436
    ///
    /// https://fontawesome.com/v5.15/icons/bowling-ball?style=solid
    #[strum(serialize = "fa-bowling-ball")]
    BowlingBall,

    /// box: \uf466
    ///
    /// https://fontawesome.com/v5.15/icons/box?style=solid
    #[strum(serialize = "fa-box")]
    Box,

    /// box-open: \uf49e
    ///
    /// https://fontawesome.com/v5.15/icons/box-open?style=solid
    #[strum(serialize = "fa-box-open")]
    BoxOpen,

    /// box-tissue: \ue05b
    ///
    /// https://fontawesome.com/v5.15/icons/box-tissue?style=solid
    #[strum(serialize = "fa-box-tissue")]
    BoxTissue,

    /// boxes: \uf468
    ///
    /// https://fontawesome.com/v5.15/icons/boxes?style=solid
    #[strum(serialize = "fa-boxes")]
    Boxes,

    /// braille: \uf2a1
    ///
    /// https://fontawesome.com/v5.15/icons/braille?style=solid
    #[strum(serialize = "fa-braille")]
    Braille,

    /// brain: \uf5dc
    ///
    /// https://fontawesome.com/v5.15/icons/brain?style=solid
    #[strum(serialize = "fa-brain")]
    Brain,

    /// bread-slice: \uf7ec
    ///
    /// https://fontawesome.com/v5.15/icons/bread-slice?style=solid
    #[strum(serialize = "fa-bread-slice")]
    BreadSlice,

    /// briefcase: \uf0b1
    ///
    /// https://fontawesome.com/v5.15/icons/briefcase?style=solid
    #[strum(serialize = "fa-briefcase")]
    Briefcase,

    /// briefcase-medical: \uf469
    ///
    /// https://fontawesome.com/v5.15/icons/briefcase-medical?style=solid
    #[strum(serialize = "fa-briefcase-medical")]
    BriefcaseMedical,

    /// broadcast-tower: \uf519
    ///
    /// https://fontawesome.com/v5.15/icons/broadcast-tower?style=solid
    #[strum(serialize = "fa-broadcast-tower")]
    BroadcastTower,

    /// broom: \uf51a
    ///
    /// https://fontawesome.com/v5.15/icons/broom?style=solid
    #[strum(serialize = "fa-broom")]
    Broom,

    /// brush: \uf55d
    ///
    /// https://fontawesome.com/v5.15/icons/brush?style=solid
    #[strum(serialize = "fa-brush")]
    Brush,

    /// bug: \uf188
    ///
    /// https://fontawesome.com/v5.15/icons/bug?style=solid
    #[strum(serialize = "fa-bug")]
    Bug,

    /// building: \uf1ad
    ///
    /// https://fontawesome.com/v5.15/icons/building?style=solid
    #[strum(serialize = "fa-building")]
    Building,

    /// bullhorn: \uf0a1
    ///
    /// https://fontawesome.com/v5.15/icons/bullhorn?style=solid
    #[strum(serialize = "fa-bullhorn")]
    Bullhorn,

    /// bullseye: \uf140
    ///
    /// https://fontawesome.com/v5.15/icons/bullseye?style=solid
    #[strum(serialize = "fa-bullseye")]
    Bullseye,

    /// burn: \uf46a
    ///
    /// https://fontawesome.com/v5.15/icons/burn?style=solid
    #[strum(serialize = "fa-burn")]
    Burn,

    /// bus: \uf207
    ///
    /// https://fontawesome.com/v5.15/icons/bus?style=solid
    #[strum(serialize = "fa-bus")]
    Bus,

    /// bus-alt: \uf55e
    ///
    /// https://fontawesome.com/v5.15/icons/bus-alt?style=solid
    #[strum(serialize = "fa-bus-alt")]
    BusAlt,

    /// business-time: \uf64a
    ///
    /// https://fontawesome.com/v5.15/icons/business-time?style=solid
    #[strum(serialize = "fa-business-time")]
    BusinessTime,

    /// calculator: \uf1ec
    ///
    /// https://fontawesome.com/v5.15/icons/calculator?style=solid
    #[strum(serialize = "fa-calculator")]
    Calculator,

    /// calendar: \uf133
    ///
    /// https://fontawesome.com/v5.15/icons/calendar?style=solid
    #[strum(serialize = "fa-calendar")]
    Calendar,

    /// calendar-alt: \uf073
    ///
    /// https://fontawesome.com/v5.15/icons/calendar-alt?style=solid
    #[strum(serialize = "fa-calendar-alt")]
    CalendarAlt,

    /// calendar-check: \uf274
    ///
    /// https://fontawesome.com/v5.15/icons/calendar-check?style=solid
    #[strum(serialize = "fa-calendar-check")]
    CalendarCheck,

    /// calendar-day: \uf783
    ///
    /// https://fontawesome.com/v5.15/icons/calendar-day?style=solid
    #[strum(serialize = "fa-calendar-day")]
    CalendarDay,

    /// calendar-minus: \uf272
    ///
    /// https://fontawesome.com/v5.15/icons/calendar-minus?style=solid
    #[strum(serialize = "fa-calendar-minus")]
    CalendarMinus,

    /// calendar-plus: \uf271
    ///
    /// https://fontawesome.com/v5.15/icons/calendar-plus?style=solid
    #[strum(serialize = "fa-calendar-plus")]
    CalendarPlus,

    /// calendar-times: \uf273
    ///
    /// https://fontawesome.com/v5.15/icons/calendar-times?style=solid
    #[strum(serialize = "fa-calendar-times")]
    CalendarTimes,

    /// calendar-week: \uf784
    ///
    /// https://fontawesome.com/v5.15/icons/calendar-week?style=solid
    #[strum(serialize = "fa-calendar-week")]
    CalendarWeek,

    /// camera: \uf030
    ///
    /// https://fontawesome.com/v5.15/icons/camera?style=solid
    #[strum(serialize = "fa-camera")]
    Camera,

    /// camera-retro: \uf083
    ///
    /// https://fontawesome.com/v5.15/icons/camera-retro?style=solid
    #[strum(serialize = "fa-camera-retro")]
    CameraRetro,

    /// campground: \uf6bb
    ///
    /// https://fontawesome.com/v5.15/icons/campground?style=solid
    #[strum(serialize = "fa-campground")]
    Campground,

    /// candy-cane: \uf786
    ///
    /// https://fontawesome.com/v5.15/icons/candy-cane?style=solid
    #[strum(serialize = "fa-candy-cane")]
    CandyCane,

    /// cannabis: \uf55f
    ///
    /// https://fontawesome.com/v5.15/icons/cannabis?style=solid
    #[strum(serialize = "fa-cannabis")]
    Cannabis,

    /// capsules: \uf46b
    ///
    /// https://fontawesome.com/v5.15/icons/capsules?style=solid
    #[strum(serialize = "fa-capsules")]
    Capsules,

    /// car: \uf1b9
    ///
    /// https://fontawesome.com/v5.15/icons/car?style=solid
    #[strum(serialize = "fa-car")]
    Car,

    /// car-alt: \uf5de
    ///
    /// https://fontawesome.com/v5.15/icons/car-alt?style=solid
    #[strum(serialize = "fa-car-alt")]
    CarAlt,

    /// car-battery: \uf5df
    ///
    /// https://fontawesome.com/v5.15/icons/car-battery?style=solid
    #[strum(serialize = "fa-car-battery")]
    CarBattery,

    /// car-crash: \uf5e1
    ///
    /// https://fontawesome.com/v5.15/icons/car-crash?style=solid
    #[strum(serialize = "fa-car-crash")]
    CarCrash,

    /// car-side: \uf5e4
    ///
    /// https://fontawesome.com/v5.15/icons/car-side?style=solid
    #[strum(serialize = "fa-car-side")]
    CarSide,

    /// caravan: \uf8ff
    ///
    /// https://fontawesome.com/v5.15/icons/caravan?style=solid
    #[strum(serialize = "fa-caravan")]
    Caravan,

    /// caret-down: \uf0d7
    ///
    /// https://fontawesome.com/v5.15/icons/caret-down?style=solid
    #[strum(serialize = "fa-caret-down")]
    CaretDown,

    /// caret-left: \uf0d9
    ///
    /// https://fontawesome.com/v5.15/icons/caret-left?style=solid
    #[strum(serialize = "fa-caret-left")]
    CaretLeft,

    /// caret-right: \uf0da
    ///
    /// https://fontawesome.com/v5.15/icons/caret-right?style=solid
    #[strum(serialize = "fa-caret-right")]
    CaretRight,

    /// caret-square-down: \uf150
    ///
    /// https://fontawesome.com/v5.15/icons/caret-square-down?style=solid
    #[strum(serialize = "fa-caret-square-down")]
    CaretSquareDown,

    /// caret-square-left: \uf191
    ///
    /// https://fontawesome.com/v5.15/icons/caret-square-left?style=solid
    #[strum(serialize = "fa-caret-square-left")]
    CaretSquareLeft,

    /// caret-square-right: \uf152
    ///
    /// https://fontawesome.com/v5.15/icons/caret-square-right?style=solid
    #[strum(serialize = "fa-caret-square-right")]
    CaretSquareRight,

    /// caret-square-up: \uf151
    ///
    /// https://fontawesome.com/v5.15/icons/caret-square-up?style=solid
    #[strum(serialize = "fa-caret-square-up")]
    CaretSquareUp,

    /// caret-up: \uf0d8
    ///
    /// https://fontawesome.com/v5.15/icons/caret-up?style=solid
    #[strum(serialize = "fa-caret-up")]
    CaretUp,

    /// carrot: \uf787
    ///
    /// https://fontawesome.com/v5.15/icons/carrot?style=solid
    #[strum(serialize = "fa-carrot")]
    Carrot,

    /// cart-arrow-down: \uf218
    ///
    /// https://fontawesome.com/v5.15/icons/cart-arrow-down?style=solid
    #[strum(serialize = "fa-cart-arrow-down")]
    CartArrowDown,

    /// cart-plus: \uf217
    ///
    /// https://fontawesome.com/v5.15/icons/cart-plus?style=solid
    #[strum(serialize = "fa-cart-plus")]
    CartPlus,

    /// cash-register: \uf788
    ///
    /// https://fontawesome.com/v5.15/icons/cash-register?style=solid
    #[strum(serialize = "fa-cash-register")]
    CashRegister,

    /// cat: \uf6be
    ///
    /// https://fontawesome.com/v5.15/icons/cat?style=solid
    #[strum(serialize = "fa-cat")]
    Cat,

    /// certificate: \uf0a3
    ///
    /// https://fontawesome.com/v5.15/icons/certificate?style=solid
    #[strum(serialize = "fa-certificate")]
    Certificate,

    /// chair: \uf6c0
    ///
    /// https://fontawesome.com/v5.15/icons/chair?style=solid
    #[strum(serialize = "fa-chair")]
    Chair,

    /// chalkboard: \uf51b
    ///
    /// https://fontawesome.com/v5.15/icons/chalkboard?style=solid
    #[strum(serialize = "fa-chalkboard")]
    Chalkboard,

    /// chalkboard-teacher: \uf51c
    ///
    /// https://fontawesome.com/v5.15/icons/chalkboard-teacher?style=solid
    #[strum(serialize = "fa-chalkboard-teacher")]
    ChalkboardTeacher,

    /// charging-station: \uf5e7
    ///
    /// https://fontawesome.com/v5.15/icons/charging-station?style=solid
    #[strum(serialize = "fa-charging-station")]
    ChargingStation,

    /// chart-area: \uf1fe
    ///
    /// https://fontawesome.com/v5.15/icons/chart-area?style=solid
    #[strum(serialize = "fa-chart-area")]
    ChartArea,

    /// chart-bar: \uf080
    ///
    /// https://fontawesome.com/v5.15/icons/chart-bar?style=solid
    #[strum(serialize = "fa-chart-bar")]
    ChartBar,

    /// chart-line: \uf201
    ///
    /// https://fontawesome.com/v5.15/icons/chart-line?style=solid
    #[strum(serialize = "fa-chart-line")]
    ChartLine,

    /// chart-pie: \uf200
    ///
    /// https://fontawesome.com/v5.15/icons/chart-pie?style=solid
    #[strum(serialize = "fa-chart-pie")]
    ChartPie,

    /// check: \uf00c
    ///
    /// https://fontawesome.com/v5.15/icons/check?style=solid
    #[strum(serialize = "fa-check")]
    Check,

    /// check-circle: \uf058
    ///
    /// https://fontawesome.com/v5.15/icons/check-circle?style=solid
    #[strum(serialize = "fa-check-circle")]
    CheckCircle,

    /// check-double: \uf560
    ///
    /// https://fontawesome.com/v5.15/icons/check-double?style=solid
    #[strum(serialize = "fa-check-double")]
    CheckDouble,

    /// check-square: \uf14a
    ///
    /// https://fontawesome.com/v5.15/icons/check-square?style=solid
    #[strum(serialize = "fa-check-square")]
    CheckSquare,

    /// cheese: \uf7ef
    ///
    /// https://fontawesome.com/v5.15/icons/cheese?style=solid
    #[strum(serialize = "fa-cheese")]
    Cheese,

    /// chess: \uf439
    ///
    /// https://fontawesome.com/v5.15/icons/chess?style=solid
    #[strum(serialize = "fa-chess")]
    Chess,

    /// chess-bishop: \uf43a
    ///
    /// https://fontawesome.com/v5.15/icons/chess-bishop?style=solid
    #[strum(serialize = "fa-chess-bishop")]
    ChessBishop,

    /// chess-board: \uf43c
    ///
    /// https://fontawesome.com/v5.15/icons/chess-board?style=solid
    #[strum(serialize = "fa-chess-board")]
    ChessBoard,

    /// chess-king: \uf43f
    ///
    /// https://fontawesome.com/v5.15/icons/chess-king?style=solid
    #[strum(serialize = "fa-chess-king")]
    ChessKing,

    /// chess-knight: \uf441
    ///
    /// https://fontawesome.com/v5.15/icons/chess-knight?style=solid
    #[strum(serialize = "fa-chess-knight")]
    ChessKnight,

    /// chess-pawn: \uf443
    ///
    /// https://fontawesome.com/v5.15/icons/chess-pawn?style=solid
    #[strum(serialize = "fa-chess-pawn")]
    ChessPawn,

    /// chess-queen: \uf445
    ///
    /// https://fontawesome.com/v5.15/icons/chess-queen?style=solid
    #[strum(serialize = "fa-chess-queen")]
    ChessQueen,

    /// chess-rook: \uf447
    ///
    /// https://fontawesome.com/v5.15/icons/chess-rook?style=solid
    #[strum(serialize = "fa-chess-rook")]
    ChessRook,

    /// chevron-circle-down: \uf13a
    ///
    /// https://fontawesome.com/v5.15/icons/chevron-circle-down?style=solid
    #[strum(serialize = "fa-chevron-circle-down")]
    ChevronCircleDown,

    /// chevron-circle-left: \uf137
    ///
    /// https://fontawesome.com/v5.15/icons/chevron-circle-left?style=solid
    #[strum(serialize = "fa-chevron-circle-left")]
    ChevronCircleLeft,

    /// chevron-circle-right: \uf138
    ///
    /// https://fontawesome.com/v5.15/icons/chevron-circle-right?style=solid
    #[strum(serialize = "fa-chevron-circle-right")]
    ChevronCircleRight,

    /// chevron-circle-up: \uf139
    ///
    /// https://fontawesome.com/v5.15/icons/chevron-circle-up?style=solid
    #[strum(serialize = "fa-chevron-circle-up")]
    ChevronCircleUp,

    /// chevron-down: \uf078
    ///
    /// https://fontawesome.com/v5.15/icons/chevron-down?style=solid
    #[strum(serialize = "fa-chevron-down")]
    ChevronDown,

    /// chevron-left: \uf053
    ///
    /// https://fontawesome.com/v5.15/icons/chevron-left?style=solid
    #[strum(serialize = "fa-chevron-left")]
    ChevronLeft,

    /// chevron-right: \uf054
    ///
    /// https://fontawesome.com/v5.15/icons/chevron-right?style=solid
    #[strum(serialize = "fa-chevron-right")]
    ChevronRight,

    /// chevron-up: \uf077
    ///
    /// https://fontawesome.com/v5.15/icons/chevron-up?style=solid
    #[strum(serialize = "fa-chevron-up")]
    ChevronUp,

    /// child: \uf1ae
    ///
    /// https://fontawesome.com/v5.15/icons/child?style=solid
    #[strum(serialize = "fa-child")]
    Child,

    /// church: \uf51d
    ///
    /// https://fontawesome.com/v5.15/icons/church?style=solid
    #[strum(serialize = "fa-church")]
    Church,

    /// circle: \uf111
    ///
    /// https://fontawesome.com/v5.15/icons/circle?style=solid
    #[strum(serialize = "fa-circle")]
    Circle,

    /// circle-notch: \uf1ce
    ///
    /// https://fontawesome.com/v5.15/icons/circle-notch?style=solid
    #[strum(serialize = "fa-circle-notch")]
    CircleNotch,

    /// city: \uf64f
    ///
    /// https://fontawesome.com/v5.15/icons/city?style=solid
    #[strum(serialize = "fa-city")]
    City,

    /// clinic-medical: \uf7f2
    ///
    /// https://fontawesome.com/v5.15/icons/clinic-medical?style=solid
    #[strum(serialize = "fa-clinic-medical")]
    ClinicMedical,

    /// clipboard: \uf328
    ///
    /// https://fontawesome.com/v5.15/icons/clipboard?style=solid
    #[strum(serialize = "fa-clipboard")]
    Clipboard,

    /// clipboard-check: \uf46c
    ///
    /// https://fontawesome.com/v5.15/icons/clipboard-check?style=solid
    #[strum(serialize = "fa-clipboard-check")]
    ClipboardCheck,

    /// clipboard-list: \uf46d
    ///
    /// https://fontawesome.com/v5.15/icons/clipboard-list?style=solid
    #[strum(serialize = "fa-clipboard-list")]
    ClipboardList,

    /// clock: \uf017
    ///
    /// https://fontawesome.com/v5.15/icons/clock?style=solid
    #[strum(serialize = "fa-clock")]
    Clock,

    /// clone: \uf24d
    ///
    /// https://fontawesome.com/v5.15/icons/clone?style=solid
    #[strum(serialize = "fa-clone")]
    Clone,

    /// closed-captioning: \uf20a
    ///
    /// https://fontawesome.com/v5.15/icons/closed-captioning?style=solid
    #[strum(serialize = "fa-closed-captioning")]
    ClosedCaptioning,

    /// cloud: \uf0c2
    ///
    /// https://fontawesome.com/v5.15/icons/cloud?style=solid
    #[strum(serialize = "fa-cloud")]
    Cloud,

    /// cloud-download-alt: \uf381
    ///
    /// https://fontawesome.com/v5.15/icons/cloud-download-alt?style=solid
    #[strum(serialize = "fa-cloud-download-alt")]
    CloudDownloadAlt,

    /// cloud-meatball: \uf73b
    ///
    /// https://fontawesome.com/v5.15/icons/cloud-meatball?style=solid
    #[strum(serialize = "fa-cloud-meatball")]
    CloudMeatball,

    /// cloud-moon: \uf6c3
    ///
    /// https://fontawesome.com/v5.15/icons/cloud-moon?style=solid
    #[strum(serialize = "fa-cloud-moon")]
    CloudMoon,

    /// cloud-moon-rain: \uf73c
    ///
    /// https://fontawesome.com/v5.15/icons/cloud-moon-rain?style=solid
    #[strum(serialize = "fa-cloud-moon-rain")]
    CloudMoonRain,

    /// cloud-rain: \uf73d
    ///
    /// https://fontawesome.com/v5.15/icons/cloud-rain?style=solid
    #[strum(serialize = "fa-cloud-rain")]
    CloudRain,

    /// cloud-showers-heavy: \uf740
    ///
    /// https://fontawesome.com/v5.15/icons/cloud-showers-heavy?style=solid
    #[strum(serialize = "fa-cloud-showers-heavy")]
    CloudShowersHeavy,

    /// cloud-sun: \uf6c4
    ///
    /// https://fontawesome.com/v5.15/icons/cloud-sun?style=solid
    #[strum(serialize = "fa-cloud-sun")]
    CloudSun,

    /// cloud-sun-rain: \uf743
    ///
    /// https://fontawesome.com/v5.15/icons/cloud-sun-rain?style=solid
    #[strum(serialize = "fa-cloud-sun-rain")]
    CloudSunRain,

    /// cloud-upload-alt: \uf382
    ///
    /// https://fontawesome.com/v5.15/icons/cloud-upload-alt?style=solid
    #[strum(serialize = "fa-cloud-upload-alt")]
    CloudUploadAlt,

    /// cocktail: \uf561
    ///
    /// https://fontawesome.com/v5.15/icons/cocktail?style=solid
    #[strum(serialize = "fa-cocktail")]
    Cocktail,

    /// code: \uf121
    ///
    /// https://fontawesome.com/v5.15/icons/code?style=solid
    #[strum(serialize = "fa-code")]
    Code,

    /// code-branch: \uf126
    ///
    /// https://fontawesome.com/v5.15/icons/code-branch?style=solid
    #[strum(serialize = "fa-code-branch")]
    CodeBranch,

    /// coffee: \uf0f4
    ///
    /// https://fontawesome.com/v5.15/icons/coffee?style=solid
    #[strum(serialize = "fa-coffee")]
    Coffee,

    /// cog: \uf013
    ///
    /// https://fontawesome.com/v5.15/icons/cog?style=solid
    #[strum(serialize = "fa-cog")]
    Cog,

    /// cogs: \uf085
    ///
    /// https://fontawesome.com/v5.15/icons/cogs?style=solid
    #[strum(serialize = "fa-cogs")]
    Cogs,

    /// coins: \uf51e
    ///
    /// https://fontawesome.com/v5.15/icons/coins?style=solid
    #[strum(serialize = "fa-coins")]
    Coins,

    /// columns: \uf0db
    ///
    /// https://fontawesome.com/v5.15/icons/columns?style=solid
    #[strum(serialize = "fa-columns")]
    Columns,

    /// comment: \uf075
    ///
    /// https://fontawesome.com/v5.15/icons/comment?style=solid
    #[strum(serialize = "fa-comment")]
    Comment,

    /// comment-alt: \uf27a
    ///
    /// https://fontawesome.com/v5.15/icons/comment-alt?style=solid
    #[strum(serialize = "fa-comment-alt")]
    CommentAlt,

    /// comment-dollar: \uf651
    ///
    /// https://fontawesome.com/v5.15/icons/comment-dollar?style=solid
    #[strum(serialize = "fa-comment-dollar")]
    CommentDollar,

    /// comment-dots: \uf4ad
    ///
    /// https://fontawesome.com/v5.15/icons/comment-dots?style=solid
    #[strum(serialize = "fa-comment-dots")]
    CommentDots,

    /// comment-medical: \uf7f5
    ///
    /// https://fontawesome.com/v5.15/icons/comment-medical?style=solid
    #[strum(serialize = "fa-comment-medical")]
    CommentMedical,

    /// comment-slash: \uf4b3
    ///
    /// https://fontawesome.com/v5.15/icons/comment-slash?style=solid
    #[strum(serialize = "fa-comment-slash")]
    CommentSlash,

    /// comments: \uf086
    ///
    /// https://fontawesome.com/v5.15/icons/comments?style=solid
    #[strum(serialize = "fa-comments")]
    Comments,

    /// comments-dollar: \uf653
    ///
    /// https://fontawesome.com/v5.15/icons/comments-dollar?style=solid
    #[strum(serialize = "fa-comments-dollar")]
    CommentsDollar,

    /// compact-disc: \uf51f
    ///
    /// https://fontawesome.com/v5.15/icons/compact-disc?style=solid
    #[strum(serialize = "fa-compact-disc")]
    CompactDisc,

    /// compass: \uf14e
    ///
    /// https://fontawesome.com/v5.15/icons/compass?style=solid
    #[strum(serialize = "fa-compass")]
    Compass,

    /// compress: \uf066
    ///
    /// https://fontawesome.com/v5.15/icons/compress?style=solid
    #[strum(serialize = "fa-compress")]
    Compress,

    /// compress-alt: \uf422
    ///
    /// https://fontawesome.com/v5.15/icons/compress-alt?style=solid
    #[strum(serialize = "fa-compress-alt")]
    CompressAlt,

    /// compress-arrows-alt: \uf78c
    ///
    /// https://fontawesome.com/v5.15/icons/compress-arrows-alt?style=solid
    #[strum(serialize = "fa-compress-arrows-alt")]
    CompressArrowsAlt,

    /// concierge-bell: \uf562
    ///
    /// https://fontawesome.com/v5.15/icons/concierge-bell?style=solid
    #[strum(serialize = "fa-concierge-bell")]
    ConciergeBell,

    /// cookie: \uf563
    ///
    /// https://fontawesome.com/v5.15/icons/cookie?style=solid
    #[strum(serialize = "fa-cookie")]
    Cookie,

    /// cookie-bite: \uf564
    ///
    /// https://fontawesome.com/v5.15/icons/cookie-bite?style=solid
    #[strum(serialize = "fa-cookie-bite")]
    CookieBite,

    /// copy: \uf0c5
    ///
    /// https://fontawesome.com/v5.15/icons/copy?style=solid
    #[strum(serialize = "fa-copy")]
    Copy,

    /// copyright: \uf1f9
    ///
    /// https://fontawesome.com/v5.15/icons/copyright?style=solid
    #[strum(serialize = "fa-copyright")]
    Copyright,

    /// couch: \uf4b8
    ///
    /// https://fontawesome.com/v5.15/icons/couch?style=solid
    #[strum(serialize = "fa-couch")]
    Couch,

    /// credit-card: \uf09d
    ///
    /// https://fontawesome.com/v5.15/icons/credit-card?style=solid
    #[strum(serialize = "fa-credit-card")]
    CreditCard,

    /// crop: \uf125
    ///
    /// https://fontawesome.com/v5.15/icons/crop?style=solid
    #[strum(serialize = "fa-crop")]
    Crop,

    /// crop-alt: \uf565
    ///
    /// https://fontawesome.com/v5.15/icons/crop-alt?style=solid
    #[strum(serialize = "fa-crop-alt")]
    CropAlt,

    /// cross: \uf654
    ///
    /// https://fontawesome.com/v5.15/icons/cross?style=solid
    #[strum(serialize = "fa-cross")]
    Cross,

    /// crosshairs: \uf05b
    ///
    /// https://fontawesome.com/v5.15/icons/crosshairs?style=solid
    #[strum(serialize = "fa-crosshairs")]
    Crosshairs,

    /// crow: \uf520
    ///
    /// https://fontawesome.com/v5.15/icons/crow?style=solid
    #[strum(serialize = "fa-crow")]
    Crow,

    /// crown: \uf521
    ///
    /// https://fontawesome.com/v5.15/icons/crown?style=solid
    #[strum(serialize = "fa-crown")]
    Crown,

    /// crutch: \uf7f7
    ///
    /// https://fontawesome.com/v5.15/icons/crutch?style=solid
    #[strum(serialize = "fa-crutch")]
    Crutch,

    /// cube: \uf1b2
    ///
    /// https://fontawesome.com/v5.15/icons/cube?style=solid
    #[strum(serialize = "fa-cube")]
    Cube,

    /// cubes: \uf1b3
    ///
    /// https://fontawesome.com/v5.15/icons/cubes?style=solid
    #[strum(serialize = "fa-cubes")]
    Cubes,

    /// cut: \uf0c4
    ///
    /// https://fontawesome.com/v5.15/icons/cut?style=solid
    #[strum(serialize = "fa-cut")]
    Cut,

    /// database: \uf1c0
    ///
    /// https://fontawesome.com/v5.15/icons/database?style=solid
    #[strum(serialize = "fa-database")]
    Database,

    /// deaf: \uf2a4
    ///
    /// https://fontawesome.com/v5.15/icons/deaf?style=solid
    #[strum(serialize = "fa-deaf")]
    Deaf,

    /// democrat: \uf747
    ///
    /// https://fontawesome.com/v5.15/icons/democrat?style=solid
    #[strum(serialize = "fa-democrat")]
    Democrat,

    /// desktop: \uf108
    ///
    /// https://fontawesome.com/v5.15/icons/desktop?style=solid
    #[strum(serialize = "fa-desktop")]
    Desktop,

    /// dharmachakra: \uf655
    ///
    /// https://fontawesome.com/v5.15/icons/dharmachakra?style=solid
    #[strum(serialize = "fa-dharmachakra")]
    Dharmachakra,

    /// diagnoses: \uf470
    ///
    /// https://fontawesome.com/v5.15/icons/diagnoses?style=solid
    #[strum(serialize = "fa-diagnoses")]
    Diagnoses,

    /// dice: \uf522
    ///
    /// https://fontawesome.com/v5.15/icons/dice?style=solid
    #[strum(serialize = "fa-dice")]
    Dice,

    /// dice-d20: \uf6cf
    ///
    /// https://fontawesome.com/v5.15/icons/dice-d20?style=solid
    #[strum(serialize = "fa-dice-d20")]
    DiceD20,

    /// dice-d6: \uf6d1
    ///
    /// https://fontawesome.com/v5.15/icons/dice-d6?style=solid
    #[strum(serialize = "fa-dice-d6")]
    DiceD6,

    /// dice-five: \uf523
    ///
    /// https://fontawesome.com/v5.15/icons/dice-five?style=solid
    #[strum(serialize = "fa-dice-five")]
    DiceFive,

    /// dice-four: \uf524
    ///
    /// https://fontawesome.com/v5.15/icons/dice-four?style=solid
    #[strum(serialize = "fa-dice-four")]
    DiceFour,

    /// dice-one: \uf525
    ///
    /// https://fontawesome.com/v5.15/icons/dice-one?style=solid
    #[strum(serialize = "fa-dice-one")]
    DiceOne,

    /// dice-six: \uf526
    ///
    /// https://fontawesome.com/v5.15/icons/dice-six?style=solid
    #[strum(serialize = "fa-dice-six")]
    DiceSix,

    /// dice-three: \uf527
    ///
    /// https://fontawesome.com/v5.15/icons/dice-three?style=solid
    #[strum(serialize = "fa-dice-three")]
    DiceThree,

    /// dice-two: \uf528
    ///
    /// https://fontawesome.com/v5.15/icons/dice-two?style=solid
    #[strum(serialize = "fa-dice-two")]
    DiceTwo,

    /// digital-tachograph: \uf566
    ///
    /// https://fontawesome.com/v5.15/icons/digital-tachograph?style=solid
    #[strum(serialize = "fa-digital-tachograph")]
    DigitalTachograph,

    /// directions: \uf5eb
    ///
    /// https://fontawesome.com/v5.15/icons/directions?style=solid
    #[strum(serialize = "fa-directions")]
    Directions,

    /// disease: \uf7fa
    ///
    /// https://fontawesome.com/v5.15/icons/disease?style=solid
    #[strum(serialize = "fa-disease")]
    Disease,

    /// divide: \uf529
    ///
    /// https://fontawesome.com/v5.15/icons/divide?style=solid
    #[strum(serialize = "fa-divide")]
    Divide,

    /// dizzy: \uf567
    ///
    /// https://fontawesome.com/v5.15/icons/dizzy?style=solid
    #[strum(serialize = "fa-dizzy")]
    Dizzy,

    /// dna: \uf471
    ///
    /// https://fontawesome.com/v5.15/icons/dna?style=solid
    #[strum(serialize = "fa-dna")]
    Dna,

    /// dog: \uf6d3
    ///
    /// https://fontawesome.com/v5.15/icons/dog?style=solid
    #[strum(serialize = "fa-dog")]
    Dog,

    /// dollar-sign: \uf155
    ///
    /// https://fontawesome.com/v5.15/icons/dollar-sign?style=solid
    #[strum(serialize = "fa-dollar-sign")]
    DollarSign,

    /// dolly: \uf472
    ///
    /// https://fontawesome.com/v5.15/icons/dolly?style=solid
    #[strum(serialize = "fa-dolly")]
    Dolly,

    /// dolly-flatbed: \uf474
    ///
    /// https://fontawesome.com/v5.15/icons/dolly-flatbed?style=solid
    #[strum(serialize = "fa-dolly-flatbed")]
    DollyFlatbed,

    /// donate: \uf4b9
    ///
    /// https://fontawesome.com/v5.15/icons/donate?style=solid
    #[strum(serialize = "fa-donate")]
    Donate,

    /// door-closed: \uf52a
    ///
    /// https://fontawesome.com/v5.15/icons/door-closed?style=solid
    #[strum(serialize = "fa-door-closed")]
    DoorClosed,

    /// door-open: \uf52b
    ///
    /// https://fontawesome.com/v5.15/icons/door-open?style=solid
    #[strum(serialize = "fa-door-open")]
    DoorOpen,

    /// dot-circle: \uf192
    ///
    /// https://fontawesome.com/v5.15/icons/dot-circle?style=solid
    #[strum(serialize = "fa-dot-circle")]
    DotCircle,

    /// dove: \uf4ba
    ///
    /// https://fontawesome.com/v5.15/icons/dove?style=solid
    #[strum(serialize = "fa-dove")]
    Dove,

    /// download: \uf019
    ///
    /// https://fontawesome.com/v5.15/icons/download?style=solid
    #[strum(serialize = "fa-download")]
    Download,

    /// drafting-compass: \uf568
    ///
    /// https://fontawesome.com/v5.15/icons/drafting-compass?style=solid
    #[strum(serialize = "fa-drafting-compass")]
    DraftingCompass,

    /// dragon: \uf6d5
    ///
    /// https://fontawesome.com/v5.15/icons/dragon?style=solid
    #[strum(serialize = "fa-dragon")]
    Dragon,

    /// draw-polygon: \uf5ee
    ///
    /// https://fontawesome.com/v5.15/icons/draw-polygon?style=solid
    #[strum(serialize = "fa-draw-polygon")]
    DrawPolygon,

    /// drum: \uf569
    ///
    /// https://fontawesome.com/v5.15/icons/drum?style=solid
    #[strum(serialize = "fa-drum")]
    Drum,

    /// drum-steelpan: \uf56a
    ///
    /// https://fontawesome.com/v5.15/icons/drum-steelpan?style=solid
    #[strum(serialize = "fa-drum-steelpan")]
    DrumSteelpan,

    /// drumstick-bite: \uf6d7
    ///
    /// https://fontawesome.com/v5.15/icons/drumstick-bite?style=solid
    #[strum(serialize = "fa-drumstick-bite")]
    DrumstickBite,

    /// dumbbell: \uf44b
    ///
    /// https://fontawesome.com/v5.15/icons/dumbbell?style=solid
    #[strum(serialize = "fa-dumbbell")]
    Dumbbell,

    /// dumpster: \uf793
    ///
    /// https://fontawesome.com/v5.15/icons/dumpster?style=solid
    #[strum(serialize = "fa-dumpster")]
    Dumpster,

    /// dumpster-fire: \uf794
    ///
    /// https://fontawesome.com/v5.15/icons/dumpster-fire?style=solid
    #[strum(serialize = "fa-dumpster-fire")]
    DumpsterFire,

    /// dungeon: \uf6d9
    ///
    /// https://fontawesome.com/v5.15/icons/dungeon?style=solid
    #[strum(serialize = "fa-dungeon")]
    Dungeon,

    /// edit: \uf044
    ///
    /// https://fontawesome.com/v5.15/icons/edit?style=solid
    #[strum(serialize = "fa-edit")]
    Edit,

    /// egg: \uf7fb
    ///
    /// https://fontawesome.com/v5.15/icons/egg?style=solid
    #[strum(serialize = "fa-egg")]
    Egg,

    /// eject: \uf052
    ///
    /// https://fontawesome.com/v5.15/icons/eject?style=solid
    #[strum(serialize = "fa-eject")]
    Eject,

    /// ellipsis-h: \uf141
    ///
    /// https://fontawesome.com/v5.15/icons/ellipsis-h?style=solid
    #[strum(serialize = "fa-ellipsis-h")]
    EllipsisH,

    /// ellipsis-v: \uf142
    ///
    /// https://fontawesome.com/v5.15/icons/ellipsis-v?style=solid
    #[strum(serialize = "fa-ellipsis-v")]
    EllipsisV,

    /// envelope: \uf0e0
    ///
    /// https://fontawesome.com/v5.15/icons/envelope?style=solid
    #[strum(serialize = "fa-envelope")]
    Envelope,

    /// envelope-open: \uf2b6
    ///
    /// https://fontawesome.com/v5.15/icons/envelope-open?style=solid
    #[strum(serialize = "fa-envelope-open")]
    EnvelopeOpen,

    /// envelope-open-text: \uf658
    ///
    /// https://fontawesome.com/v5.15/icons/envelope-open-text?style=solid
    #[strum(serialize = "fa-envelope-open-text")]
    EnvelopeOpenText,

    /// envelope-square: \uf199
    ///
    /// https://fontawesome.com/v5.15/icons/envelope-square?style=solid
    #[strum(serialize = "fa-envelope-square")]
    EnvelopeSquare,

    /// equals: \uf52c
    ///
    /// https://fontawesome.com/v5.15/icons/equals?style=solid
    #[strum(serialize = "fa-equals")]
    Equals,

    /// eraser: \uf12d
    ///
    /// https://fontawesome.com/v5.15/icons/eraser?style=solid
    #[strum(serialize = "fa-eraser")]
    Eraser,

    /// ethernet: \uf796
    ///
    /// https://fontawesome.com/v5.15/icons/ethernet?style=solid
    #[strum(serialize = "fa-ethernet")]
    Ethernet,

    /// euro-sign: \uf153
    ///
    /// https://fontawesome.com/v5.15/icons/euro-sign?style=solid
    #[strum(serialize = "fa-euro-sign")]
    EuroSign,

    /// exchange-alt: \uf362
    ///
    /// https://fontawesome.com/v5.15/icons/exchange-alt?style=solid
    #[strum(serialize = "fa-exchange-alt")]
    ExchangeAlt,

    /// exclamation: \uf12a
    ///
    /// https://fontawesome.com/v5.15/icons/exclamation?style=solid
    #[strum(serialize = "fa-exclamation")]
    Exclamation,

    /// exclamation-circle: \uf06a
    ///
    /// https://fontawesome.com/v5.15/icons/exclamation-circle?style=solid
    #[strum(serialize = "fa-exclamation-circle")]
    ExclamationCircle,

    /// exclamation-triangle: \uf071
    ///
    /// https://fontawesome.com/v5.15/icons/exclamation-triangle?style=solid
    #[strum(serialize = "fa-exclamation-triangle")]
    ExclamationTriangle,

    /// expand: \uf065
    ///
    /// https://fontawesome.com/v5.15/icons/expand?style=solid
    #[strum(serialize = "fa-expand")]
    Expand,

    /// expand-alt: \uf424
    ///
    /// https://fontawesome.com/v5.15/icons/expand-alt?style=solid
    #[strum(serialize = "fa-expand-alt")]
    ExpandAlt,

    /// expand-arrows-alt: \uf31e
    ///
    /// https://fontawesome.com/v5.15/icons/expand-arrows-alt?style=solid
    #[strum(serialize = "fa-expand-arrows-alt")]
    ExpandArrowsAlt,

    /// external-link-alt: \uf35d
    ///
    /// https://fontawesome.com/v5.15/icons/external-link-alt?style=solid
    #[strum(serialize = "fa-external-link-alt")]
    ExternalLinkAlt,

    /// external-link-square-alt: \uf360
    ///
    /// https://fontawesome.com/v5.15/icons/external-link-square-alt?style=solid
    #[strum(serialize = "fa-external-link-square-alt")]
    ExternalLinkSquareAlt,

    /// eye: \uf06e
    ///
    /// https://fontawesome.com/v5.15/icons/eye?style=solid
    #[strum(serialize = "fa-eye")]
    Eye,

    /// eye-dropper: \uf1fb
    ///
    /// https://fontawesome.com/v5.15/icons/eye-dropper?style=solid
    #[strum(serialize = "fa-eye-dropper")]
    EyeDropper,

    /// eye-slash: \uf070
    ///
    /// https://fontawesome.com/v5.15/icons/eye-slash?style=solid
    #[strum(serialize = "fa-eye-slash")]
    EyeSlash,

    /// fan: \uf863
    ///
    /// https://fontawesome.com/v5.15/icons/fan?style=solid
    #[strum(serialize = "fa-fan")]
    Fan,

    /// fast-backward: \uf049
    ///
    /// https://fontawesome.com/v5.15/icons/fast-backward?style=solid
    #[strum(serialize = "fa-fast-backward")]
    FastBackward,

    /// fast-forward: \uf050
    ///
    /// https://fontawesome.com/v5.15/icons/fast-forward?style=solid
    #[strum(serialize = "fa-fast-forward")]
    FastForward,

    /// faucet: \ue005
    ///
    /// https://fontawesome.com/v5.15/icons/faucet?style=solid
    #[strum(serialize = "fa-faucet")]
    Faucet,

    /// fax: \uf1ac
    ///
    /// https://fontawesome.com/v5.15/icons/fax?style=solid
    #[strum(serialize = "fa-fax")]
    Fax,

    /// feather: \uf52d
    ///
    /// https://fontawesome.com/v5.15/icons/feather?style=solid
    #[strum(serialize = "fa-feather")]
    Feather,

    /// feather-alt: \uf56b
    ///
    /// https://fontawesome.com/v5.15/icons/feather-alt?style=solid
    #[strum(serialize = "fa-feather-alt")]
    FeatherAlt,

    /// female: \uf182
    ///
    /// https://fontawesome.com/v5.15/icons/female?style=solid
    #[strum(serialize = "fa-female")]
    Female,

    /// fighter-jet: \uf0fb
    ///
    /// https://fontawesome.com/v5.15/icons/fighter-jet?style=solid
    #[strum(serialize = "fa-fighter-jet")]
    FighterJet,

    /// file: \uf15b
    ///
    /// https://fontawesome.com/v5.15/icons/file?style=solid
    #[strum(serialize = "fa-file")]
    File,

    /// file-alt: \uf15c
    ///
    /// https://fontawesome.com/v5.15/icons/file-alt?style=solid
    #[strum(serialize = "fa-file-alt")]
    FileAlt,

    /// file-archive: \uf1c6
    ///
    /// https://fontawesome.com/v5.15/icons/file-archive?style=solid
    #[strum(serialize = "fa-file-archive")]
    FileArchive,

    /// file-audio: \uf1c7
    ///
    /// https://fontawesome.com/v5.15/icons/file-audio?style=solid
    #[strum(serialize = "fa-file-audio")]
    FileAudio,

    /// file-code: \uf1c9
    ///
    /// https://fontawesome.com/v5.15/icons/file-code?style=solid
    #[strum(serialize = "fa-file-code")]
    FileCode,

    /// file-contract: \uf56c
    ///
    /// https://fontawesome.com/v5.15/icons/file-contract?style=solid
    #[strum(serialize = "fa-file-contract")]
    FileContract,

    /// file-csv: \uf6dd
    ///
    /// https://fontawesome.com/v5.15/icons/file-csv?style=solid
    #[strum(serialize = "fa-file-csv")]
    FileCsv,

    /// file-download: \uf56d
    ///
    /// https://fontawesome.com/v5.15/icons/file-download?style=solid
    #[strum(serialize = "fa-file-download")]
    FileDownload,

    /// file-excel: \uf1c3
    ///
    /// https://fontawesome.com/v5.15/icons/file-excel?style=solid
    #[strum(serialize = "fa-file-excel")]
    FileExcel,

    /// file-export: \uf56e
    ///
    /// https://fontawesome.com/v5.15/icons/file-export?style=solid
    #[strum(serialize = "fa-file-export")]
    FileExport,

    /// file-image: \uf1c5
    ///
    /// https://fontawesome.com/v5.15/icons/file-image?style=solid
    #[strum(serialize = "fa-file-image")]
    FileImage,

    /// file-import: \uf56f
    ///
    /// https://fontawesome.com/v5.15/icons/file-import?style=solid
    #[strum(serialize = "fa-file-import")]
    FileImport,

    /// file-invoice: \uf570
    ///
    /// https://fontawesome.com/v5.15/icons/file-invoice?style=solid
    #[strum(serialize = "fa-file-invoice")]
    FileInvoice,

    /// file-invoice-dollar: \uf571
    ///
    /// https://fontawesome.com/v5.15/icons/file-invoice-dollar?style=solid
    #[strum(serialize = "fa-file-invoice-dollar")]
    FileInvoiceDollar,

    /// file-medical: \uf477
    ///
    /// https://fontawesome.com/v5.15/icons/file-medical?style=solid
    #[strum(serialize = "fa-file-medical")]
    FileMedical,

    /// file-medical-alt: \uf478
    ///
    /// https://fontawesome.com/v5.15/icons/file-medical-alt?style=solid
    #[strum(serialize = "fa-file-medical-alt")]
    FileMedicalAlt,

    /// file-pdf: \uf1c1
    ///
    /// https://fontawesome.com/v5.15/icons/file-pdf?style=solid
    #[strum(serialize = "fa-file-pdf")]
    FilePdf,

    /// file-powerpoint: \uf1c4
    ///
    /// https://fontawesome.com/v5.15/icons/file-powerpoint?style=solid
    #[strum(serialize = "fa-file-powerpoint")]
    FilePowerpoint,

    /// file-prescription: \uf572
    ///
    /// https://fontawesome.com/v5.15/icons/file-prescription?style=solid
    #[strum(serialize = "fa-file-prescription")]
    FilePrescription,

    /// file-signature: \uf573
    ///
    /// https://fontawesome.com/v5.15/icons/file-signature?style=solid
    #[strum(serialize = "fa-file-signature")]
    FileSignature,

    /// file-upload: \uf574
    ///
    /// https://fontawesome.com/v5.15/icons/file-upload?style=solid
    #[strum(serialize = "fa-file-upload")]
    FileUpload,

    /// file-video: \uf1c8
    ///
    /// https://fontawesome.com/v5.15/icons/file-video?style=solid
    #[strum(serialize = "fa-file-video")]
    FileVideo,

    /// file-word: \uf1c2
    ///
    /// https://fontawesome.com/v5.15/icons/file-word?style=solid
    #[strum(serialize = "fa-file-word")]
    FileWord,

    /// fill: \uf575
    ///
    /// https://fontawesome.com/v5.15/icons/fill?style=solid
    #[strum(serialize = "fa-fill")]
    Fill,

    /// fill-drip: \uf576
    ///
    /// https://fontawesome.com/v5.15/icons/fill-drip?style=solid
    #[strum(serialize = "fa-fill-drip")]
    FillDrip,

    /// film: \uf008
    ///
    /// https://fontawesome.com/v5.15/icons/film?style=solid
    #[strum(serialize = "fa-film")]
    Film,

    /// filter: \uf0b0
    ///
    /// https://fontawesome.com/v5.15/icons/filter?style=solid
    #[strum(serialize = "fa-filter")]
    Filter,

    /// fingerprint: \uf577
    ///
    /// https://fontawesome.com/v5.15/icons/fingerprint?style=solid
    #[strum(serialize = "fa-fingerprint")]
    Fingerprint,

    /// fire: \uf06d
    ///
    /// https://fontawesome.com/v5.15/icons/fire?style=solid
    #[strum(serialize = "fa-fire")]
    Fire,

    /// fire-alt: \uf7e4
    ///
    /// https://fontawesome.com/v5.15/icons/fire-alt?style=solid
    #[strum(serialize = "fa-fire-alt")]
    FireAlt,

    /// fire-extinguisher: \uf134
    ///
    /// https://fontawesome.com/v5.15/icons/fire-extinguisher?style=solid
    #[strum(serialize = "fa-fire-extinguisher")]
    FireExtinguisher,

    /// first-aid: \uf479
    ///
    /// https://fontawesome.com/v5.15/icons/first-aid?style=solid
    #[strum(serialize = "fa-first-aid")]
    FirstAid,

    /// fish: \uf578
    ///
    /// https://fontawesome.com/v5.15/icons/fish?style=solid
    #[strum(serialize = "fa-fish")]
    Fish,

    /// fist-raised: \uf6de
    ///
    /// https://fontawesome.com/v5.15/icons/fist-raised?style=solid
    #[strum(serialize = "fa-fist-raised")]
    FistRaised,

    /// flag: \uf024
    ///
    /// https://fontawesome.com/v5.15/icons/flag?style=solid
    #[strum(serialize = "fa-flag")]
    Flag,

    /// flag-checkered: \uf11e
    ///
    /// https://fontawesome.com/v5.15/icons/flag-checkered?style=solid
    #[strum(serialize = "fa-flag-checkered")]
    FlagCheckered,

    /// flag-usa: \uf74d
    ///
    /// https://fontawesome.com/v5.15/icons/flag-usa?style=solid
    #[strum(serialize = "fa-flag-usa")]
    FlagUsa,

    /// flask: \uf0c3
    ///
    /// https://fontawesome.com/v5.15/icons/flask?style=solid
    #[strum(serialize = "fa-flask")]
    Flask,

    /// flushed: \uf579
    ///
    /// https://fontawesome.com/v5.15/icons/flushed?style=solid
    #[strum(serialize = "fa-flushed")]
    Flushed,

    /// folder: \uf07b
    ///
    /// https://fontawesome.com/v5.15/icons/folder?style=solid
    #[strum(serialize = "fa-folder")]
    Folder,

    /// folder-minus: \uf65d
    ///
    /// https://fontawesome.com/v5.15/icons/folder-minus?style=solid
    #[strum(serialize = "fa-folder-minus")]
    FolderMinus,

    /// folder-open: \uf07c
    ///
    /// https://fontawesome.com/v5.15/icons/folder-open?style=solid
    #[strum(serialize = "fa-folder-open")]
    FolderOpen,

    /// folder-plus: \uf65e
    ///
    /// https://fontawesome.com/v5.15/icons/folder-plus?style=solid
    #[strum(serialize = "fa-folder-plus")]
    FolderPlus,

    /// font: \uf031
    ///
    /// https://fontawesome.com/v5.15/icons/font?style=solid
    #[strum(serialize = "fa-font")]
    Font,

    /// football-ball: \uf44e
    ///
    /// https://fontawesome.com/v5.15/icons/football-ball?style=solid
    #[strum(serialize = "fa-football-ball")]
    FootballBall,

    /// forward: \uf04e
    ///
    /// https://fontawesome.com/v5.15/icons/forward?style=solid
    #[strum(serialize = "fa-forward")]
    Forward,

    /// frog: \uf52e
    ///
    /// https://fontawesome.com/v5.15/icons/frog?style=solid
    #[strum(serialize = "fa-frog")]
    Frog,

    /// frown: \uf119
    ///
    /// https://fontawesome.com/v5.15/icons/frown?style=solid
    #[strum(serialize = "fa-frown")]
    Frown,

    /// frown-open: \uf57a
    ///
    /// https://fontawesome.com/v5.15/icons/frown-open?style=solid
    #[strum(serialize = "fa-frown-open")]
    FrownOpen,

    /// funnel-dollar: \uf662
    ///
    /// https://fontawesome.com/v5.15/icons/funnel-dollar?style=solid
    #[strum(serialize = "fa-funnel-dollar")]
    FunnelDollar,

    /// futbol: \uf1e3
    ///
    /// https://fontawesome.com/v5.15/icons/futbol?style=solid
    #[strum(serialize = "fa-futbol")]
    Futbol,

    /// gamepad: \uf11b
    ///
    /// https://fontawesome.com/v5.15/icons/gamepad?style=solid
    #[strum(serialize = "fa-gamepad")]
    Gamepad,

    /// gas-pump: \uf52f
    ///
    /// https://fontawesome.com/v5.15/icons/gas-pump?style=solid
    #[strum(serialize = "fa-gas-pump")]
    GasPump,

    /// gavel: \uf0e3
    ///
    /// https://fontawesome.com/v5.15/icons/gavel?style=solid
    #[strum(serialize = "fa-gavel")]
    Gavel,

    /// gem: \uf3a5
    ///
    /// https://fontawesome.com/v5.15/icons/gem?style=solid
    #[strum(serialize = "fa-gem")]
    Gem,

    /// genderless: \uf22d
    ///
    /// https://fontawesome.com/v5.15/icons/genderless?style=solid
    #[strum(serialize = "fa-genderless")]
    Genderless,

    /// ghost: \uf6e2
    ///
    /// https://fontawesome.com/v5.15/icons/ghost?style=solid
    #[strum(serialize = "fa-ghost")]
    Ghost,

    /// gift: \uf06b
    ///
    /// https://fontawesome.com/v5.15/icons/gift?style=solid
    #[strum(serialize = "fa-gift")]
    Gift,

    /// gifts: \uf79c
    ///
    /// https://fontawesome.com/v5.15/icons/gifts?style=solid
    #[strum(serialize = "fa-gifts")]
    Gifts,

    /// glass-cheers: \uf79f
    ///
    /// https://fontawesome.com/v5.15/icons/glass-cheers?style=solid
    #[strum(serialize = "fa-glass-cheers")]
    GlassCheers,

    /// glass-martini: \uf000
    ///
    /// https://fontawesome.com/v5.15/icons/glass-martini?style=solid
    #[strum(serialize = "fa-glass-martini")]
    GlassMartini,

    /// glass-martini-alt: \uf57b
    ///
    /// https://fontawesome.com/v5.15/icons/glass-martini-alt?style=solid
    #[strum(serialize = "fa-glass-martini-alt")]
    GlassMartiniAlt,

    /// glass-whiskey: \uf7a0
    ///
    /// https://fontawesome.com/v5.15/icons/glass-whiskey?style=solid
    #[strum(serialize = "fa-glass-whiskey")]
    GlassWhiskey,

    /// glasses: \uf530
    ///
    /// https://fontawesome.com/v5.15/icons/glasses?style=solid
    #[strum(serialize = "fa-glasses")]
    Glasses,

    /// globe: \uf0ac
    ///
    /// https://fontawesome.com/v5.15/icons/globe?style=solid
    #[strum(serialize = "fa-globe")]
    Globe,

    /// globe-africa: \uf57c
    ///
    /// https://fontawesome.com/v5.15/icons/globe-africa?style=solid
    #[strum(serialize = "fa-globe-africa")]
    GlobeAfrica,

    /// globe-americas: \uf57d
    ///
    /// https://fontawesome.com/v5.15/icons/globe-americas?style=solid
    #[strum(serialize = "fa-globe-americas")]
    GlobeAmericas,

    /// globe-asia: \uf57e
    ///
    /// https://fontawesome.com/v5.15/icons/globe-asia?style=solid
    #[strum(serialize = "fa-globe-asia")]
    GlobeAsia,

    /// globe-europe: \uf7a2
    ///
    /// https://fontawesome.com/v5.15/icons/globe-europe?style=solid
    #[strum(serialize = "fa-globe-europe")]
    GlobeEurope,

    /// golf-ball: \uf450
    ///
    /// https://fontawesome.com/v5.15/icons/golf-ball?style=solid
    #[strum(serialize = "fa-golf-ball")]
    GolfBall,

    /// gopuram: \uf664
    ///
    /// https://fontawesome.com/v5.15/icons/gopuram?style=solid
    #[strum(serialize = "fa-gopuram")]
    Gopuram,

    /// graduation-cap: \uf19d
    ///
    /// https://fontawesome.com/v5.15/icons/graduation-cap?style=solid
    #[strum(serialize = "fa-graduation-cap")]
    GraduationCap,

    /// greater-than: \uf531
    ///
    /// https://fontawesome.com/v5.15/icons/greater-than?style=solid
    #[strum(serialize = "fa-greater-than")]
    GreaterThan,

    /// greater-than-equal: \uf532
    ///
    /// https://fontawesome.com/v5.15/icons/greater-than-equal?style=solid
    #[strum(serialize = "fa-greater-than-equal")]
    GreaterThanEqual,

    /// grimace: \uf57f
    ///
    /// https://fontawesome.com/v5.15/icons/grimace?style=solid
    #[strum(serialize = "fa-grimace")]
    Grimace,

    /// grin: \uf580
    ///
    /// https://fontawesome.com/v5.15/icons/grin?style=solid
    #[strum(serialize = "fa-grin")]
    Grin,

    /// grin-alt: \uf581
    ///
    /// https://fontawesome.com/v5.15/icons/grin-alt?style=solid
    #[strum(serialize = "fa-grin-alt")]
    GrinAlt,

    /// grin-beam: \uf582
    ///
    /// https://fontawesome.com/v5.15/icons/grin-beam?style=solid
    #[strum(serialize = "fa-grin-beam")]
    GrinBeam,

    /// grin-beam-sweat: \uf583
    ///
    /// https://fontawesome.com/v5.15/icons/grin-beam-sweat?style=solid
    #[strum(serialize = "fa-grin-beam-sweat")]
    GrinBeamSweat,

    /// grin-hearts: \uf584
    ///
    /// https://fontawesome.com/v5.15/icons/grin-hearts?style=solid
    #[strum(serialize = "fa-grin-hearts")]
    GrinHearts,

    /// grin-squint: \uf585
    ///
    /// https://fontawesome.com/v5.15/icons/grin-squint?style=solid
    #[strum(serialize = "fa-grin-squint")]
    GrinSquint,

    /// grin-squint-tears: \uf586
    ///
    /// https://fontawesome.com/v5.15/icons/grin-squint-tears?style=solid
    #[strum(serialize = "fa-grin-squint-tears")]
    GrinSquintTears,

    /// grin-stars: \uf587
    ///
    /// https://fontawesome.com/v5.15/icons/grin-stars?style=solid
    #[strum(serialize = "fa-grin-stars")]
    GrinStars,

    /// grin-tears: \uf588
    ///
    /// https://fontawesome.com/v5.15/icons/grin-tears?style=solid
    #[strum(serialize = "fa-grin-tears")]
    GrinTears,

    /// grin-tongue: \uf589
    ///
    /// https://fontawesome.com/v5.15/icons/grin-tongue?style=solid
    #[strum(serialize = "fa-grin-tongue")]
    GrinTongue,

    /// grin-tongue-squint: \uf58a
    ///
    /// https://fontawesome.com/v5.15/icons/grin-tongue-squint?style=solid
    #[strum(serialize = "fa-grin-tongue-squint")]
    GrinTongueSquint,

    /// grin-tongue-wink: \uf58b
    ///
    /// https://fontawesome.com/v5.15/icons/grin-tongue-wink?style=solid
    #[strum(serialize = "fa-grin-tongue-wink")]
    GrinTongueWink,

    /// grin-wink: \uf58c
    ///
    /// https://fontawesome.com/v5.15/icons/grin-wink?style=solid
    #[strum(serialize = "fa-grin-wink")]
    GrinWink,

    /// grip-horizontal: \uf58d
    ///
    /// https://fontawesome.com/v5.15/icons/grip-horizontal?style=solid
    #[strum(serialize = "fa-grip-horizontal")]
    GripHorizontal,

    /// grip-lines: \uf7a4
    ///
    /// https://fontawesome.com/v5.15/icons/grip-lines?style=solid
    #[strum(serialize = "fa-grip-lines")]
    GripLines,

    /// grip-lines-vertical: \uf7a5
    ///
    /// https://fontawesome.com/v5.15/icons/grip-lines-vertical?style=solid
    #[strum(serialize = "fa-grip-lines-vertical")]
    GripLinesVertical,

    /// grip-vertical: \uf58e
    ///
    /// https://fontawesome.com/v5.15/icons/grip-vertical?style=solid
    #[strum(serialize = "fa-grip-vertical")]
    GripVertical,

    /// guitar: \uf7a6
    ///
    /// https://fontawesome.com/v5.15/icons/guitar?style=solid
    #[strum(serialize = "fa-guitar")]
    Guitar,

    /// h-square: \uf0fd
    ///
    /// https://fontawesome.com/v5.15/icons/h-square?style=solid
    #[strum(serialize = "fa-h-square")]
    HSquare,

    /// hamburger: \uf805
    ///
    /// https://fontawesome.com/v5.15/icons/hamburger?style=solid
    #[strum(serialize = "fa-hamburger")]
    Hamburger,

    /// hammer: \uf6e3
    ///
    /// https://fontawesome.com/v5.15/icons/hammer?style=solid
    #[strum(serialize = "fa-hammer")]
    Hammer,

    /// hamsa: \uf665
    ///
    /// https://fontawesome.com/v5.15/icons/hamsa?style=solid
    #[strum(serialize = "fa-hamsa")]
    Hamsa,

    /// hand-holding: \uf4bd
    ///
    /// https://fontawesome.com/v5.15/icons/hand-holding?style=solid
    #[strum(serialize = "fa-hand-holding")]
    HandHolding,

    /// hand-holding-heart: \uf4be
    ///
    /// https://fontawesome.com/v5.15/icons/hand-holding-heart?style=solid
    #[strum(serialize = "fa-hand-holding-heart")]
    HandHoldingHeart,

    /// hand-holding-medical: \ue05c
    ///
    /// https://fontawesome.com/v5.15/icons/hand-holding-medical?style=solid
    #[strum(serialize = "fa-hand-holding-medical")]
    HandHoldingMedical,

    /// hand-holding-usd: \uf4c0
    ///
    /// https://fontawesome.com/v5.15/icons/hand-holding-usd?style=solid
    #[strum(serialize = "fa-hand-holding-usd")]
    HandHoldingUsd,

    /// hand-holding-water: \uf4c1
    ///
    /// https://fontawesome.com/v5.15/icons/hand-holding-water?style=solid
    #[strum(serialize = "fa-hand-holding-water")]
    HandHoldingWater,

    /// hand-lizard: \uf258
    ///
    /// https://fontawesome.com/v5.15/icons/hand-lizard?style=solid
    #[strum(serialize = "fa-hand-lizard")]
    HandLizard,

    /// hand-middle-finger: \uf806
    ///
    /// https://fontawesome.com/v5.15/icons/hand-middle-finger?style=solid
    #[strum(serialize = "fa-hand-middle-finger")]
    HandMiddleFinger,

    /// hand-paper: \uf256
    ///
    /// https://fontawesome.com/v5.15/icons/hand-paper?style=solid
    #[strum(serialize = "fa-hand-paper")]
    HandPaper,

    /// hand-peace: \uf25b
    ///
    /// https://fontawesome.com/v5.15/icons/hand-peace?style=solid
    #[strum(serialize = "fa-hand-peace")]
    HandPeace,

    /// hand-point-down: \uf0a7
    ///
    /// https://fontawesome.com/v5.15/icons/hand-point-down?style=solid
    #[strum(serialize = "fa-hand-point-down")]
    HandPointDown,

    /// hand-point-left: \uf0a5
    ///
    /// https://fontawesome.com/v5.15/icons/hand-point-left?style=solid
    #[strum(serialize = "fa-hand-point-left")]
    HandPointLeft,

    /// hand-point-right: \uf0a4
    ///
    /// https://fontawesome.com/v5.15/icons/hand-point-right?style=solid
    #[strum(serialize = "fa-hand-point-right")]
    HandPointRight,

    /// hand-point-up: \uf0a6
    ///
    /// https://fontawesome.com/v5.15/icons/hand-point-up?style=solid
    #[strum(serialize = "fa-hand-point-up")]
    HandPointUp,

    /// hand-pointer: \uf25a
    ///
    /// https://fontawesome.com/v5.15/icons/hand-pointer?style=solid
    #[strum(serialize = "fa-hand-pointer")]
    HandPointer,

    /// hand-rock: \uf255
    ///
    /// https://fontawesome.com/v5.15/icons/hand-rock?style=solid
    #[strum(serialize = "fa-hand-rock")]
    HandRock,

    /// hand-scissors: \uf257
    ///
    /// https://fontawesome.com/v5.15/icons/hand-scissors?style=solid
    #[strum(serialize = "fa-hand-scissors")]
    HandScissors,

    /// hand-sparkles: \ue05d
    ///
    /// https://fontawesome.com/v5.15/icons/hand-sparkles?style=solid
    #[strum(serialize = "fa-hand-sparkles")]
    HandSparkles,

    /// hand-spock: \uf259
    ///
    /// https://fontawesome.com/v5.15/icons/hand-spock?style=solid
    #[strum(serialize = "fa-hand-spock")]
    HandSpock,

    /// hands: \uf4c2
    ///
    /// https://fontawesome.com/v5.15/icons/hands?style=solid
    #[strum(serialize = "fa-hands")]
    Hands,

    /// hands-helping: \uf4c4
    ///
    /// https://fontawesome.com/v5.15/icons/hands-helping?style=solid
    #[strum(serialize = "fa-hands-helping")]
    HandsHelping,

    /// hands-wash: \ue05e
    ///
    /// https://fontawesome.com/v5.15/icons/hands-wash?style=solid
    #[strum(serialize = "fa-hands-wash")]
    HandsWash,

    /// handshake: \uf2b5
    ///
    /// https://fontawesome.com/v5.15/icons/handshake?style=solid
    #[strum(serialize = "fa-handshake")]
    Handshake,

    /// handshake-alt-slash: \ue05f
    ///
    /// https://fontawesome.com/v5.15/icons/handshake-alt-slash?style=solid
    #[strum(serialize = "fa-handshake-alt-slash")]
    HandshakeAltSlash,

    /// handshake-slash: \ue060
    ///
    /// https://fontawesome.com/v5.15/icons/handshake-slash?style=solid
    #[strum(serialize = "fa-handshake-slash")]
    HandshakeSlash,

    /// hanukiah: \uf6e6
    ///
    /// https://fontawesome.com/v5.15/icons/hanukiah?style=solid
    #[strum(serialize = "fa-hanukiah")]
    Hanukiah,

    /// hard-hat: \uf807
    ///
    /// https://fontawesome.com/v5.15/icons/hard-hat?style=solid
    #[strum(serialize = "fa-hard-hat")]
    HardHat,

    /// hashtag: \uf292
    ///
    /// https://fontawesome.com/v5.15/icons/hashtag?style=solid
    #[strum(serialize = "fa-hashtag")]
    Hashtag,

    /// hat-cowboy: \uf8c0
    ///
    /// https://fontawesome.com/v5.15/icons/hat-cowboy?style=solid
    #[strum(serialize = "fa-hat-cowboy")]
    HatCowboy,

    /// hat-cowboy-side: \uf8c1
    ///
    /// https://fontawesome.com/v5.15/icons/hat-cowboy-side?style=solid
    #[strum(serialize = "fa-hat-cowboy-side")]
    HatCowboySide,

    /// hat-wizard: \uf6e8
    ///
    /// https://fontawesome.com/v5.15/icons/hat-wizard?style=solid
    #[strum(serialize = "fa-hat-wizard")]
    HatWizard,

    /// hdd: \uf0a0
    ///
    /// https://fontawesome.com/v5.15/icons/hdd?style=solid
    #[strum(serialize = "fa-hdd")]
    Hdd,

    /// head-side-cough: \ue061
    ///
    /// https://fontawesome.com/v5.15/icons/head-side-cough?style=solid
    #[strum(serialize = "fa-head-side-cough")]
    HeadSideCough,

    /// head-side-cough-slash: \ue062
    ///
    /// https://fontawesome.com/v5.15/icons/head-side-cough-slash?style=solid
    #[strum(serialize = "fa-head-side-cough-slash")]
    HeadSideCoughSlash,

    /// head-side-mask: \ue063
    ///
    /// https://fontawesome.com/v5.15/icons/head-side-mask?style=solid
    #[strum(serialize = "fa-head-side-mask")]
    HeadSideMask,

    /// head-side-virus: \ue064
    ///
    /// https://fontawesome.com/v5.15/icons/head-side-virus?style=solid
    #[strum(serialize = "fa-head-side-virus")]
    HeadSideVirus,

    /// heading: \uf1dc
    ///
    /// https://fontawesome.com/v5.15/icons/heading?style=solid
    #[strum(serialize = "fa-heading")]
    Heading,

    /// headphones: \uf025
    ///
    /// https://fontawesome.com/v5.15/icons/headphones?style=solid
    #[strum(serialize = "fa-headphones")]
    Headphones,

    /// headphones-alt: \uf58f
    ///
    /// https://fontawesome.com/v5.15/icons/headphones-alt?style=solid
    #[strum(serialize = "fa-headphones-alt")]
    HeadphonesAlt,

    /// headset: \uf590
    ///
    /// https://fontawesome.com/v5.15/icons/headset?style=solid
    #[strum(serialize = "fa-headset")]
    Headset,

    /// heart: \uf004
    ///
    /// https://fontawesome.com/v5.15/icons/heart?style=solid
    #[strum(serialize = "fa-heart")]
    Heart,

    /// heart-broken: \uf7a9
    ///
    /// https://fontawesome.com/v5.15/icons/heart-broken?style=solid
    #[strum(serialize = "fa-heart-broken")]
    HeartBroken,

    /// heartbeat: \uf21e
    ///
    /// https://fontawesome.com/v5.15/icons/heartbeat?style=solid
    #[strum(serialize = "fa-heartbeat")]
    Heartbeat,

    /// helicopter: \uf533
    ///
    /// https://fontawesome.com/v5.15/icons/helicopter?style=solid
    #[strum(serialize = "fa-helicopter")]
    Helicopter,

    /// highlighter: \uf591
    ///
    /// https://fontawesome.com/v5.15/icons/highlighter?style=solid
    #[strum(serialize = "fa-highlighter")]
    Highlighter,

    /// hiking: \uf6ec
    ///
    /// https://fontawesome.com/v5.15/icons/hiking?style=solid
    #[strum(serialize = "fa-hiking")]
    Hiking,

    /// hippo: \uf6ed
    ///
    /// https://fontawesome.com/v5.15/icons/hippo?style=solid
    #[strum(serialize = "fa-hippo")]
    Hippo,

    /// history: \uf1da
    ///
    /// https://fontawesome.com/v5.15/icons/history?style=solid
    #[strum(serialize = "fa-history")]
    History,

    /// hockey-puck: \uf453
    ///
    /// https://fontawesome.com/v5.15/icons/hockey-puck?style=solid
    #[strum(serialize = "fa-hockey-puck")]
    HockeyPuck,

    /// holly-berry: \uf7aa
    ///
    /// https://fontawesome.com/v5.15/icons/holly-berry?style=solid
    #[strum(serialize = "fa-holly-berry")]
    HollyBerry,

    /// home: \uf015
    ///
    /// https://fontawesome.com/v5.15/icons/home?style=solid
    #[strum(serialize = "fa-home")]
    Home,

    /// horse: \uf6f0
    ///
    /// https://fontawesome.com/v5.15/icons/horse?style=solid
    #[strum(serialize = "fa-horse")]
    Horse,

    /// horse-head: \uf7ab
    ///
    /// https://fontawesome.com/v5.15/icons/horse-head?style=solid
    #[strum(serialize = "fa-horse-head")]
    HorseHead,

    /// hospital: \uf0f8
    ///
    /// https://fontawesome.com/v5.15/icons/hospital?style=solid
    #[strum(serialize = "fa-hospital")]
    Hospital,

    /// hospital-alt: \uf47d
    ///
    /// https://fontawesome.com/v5.15/icons/hospital-alt?style=solid
    #[strum(serialize = "fa-hospital-alt")]
    HospitalAlt,

    /// hospital-symbol: \uf47e
    ///
    /// https://fontawesome.com/v5.15/icons/hospital-symbol?style=solid
    #[strum(serialize = "fa-hospital-symbol")]
    HospitalSymbol,

    /// hospital-user: \uf80d
    ///
    /// https://fontawesome.com/v5.15/icons/hospital-user?style=solid
    #[strum(serialize = "fa-hospital-user")]
    HospitalUser,

    /// hot-tub: \uf593
    ///
    /// https://fontawesome.com/v5.15/icons/hot-tub?style=solid
    #[strum(serialize = "fa-hot-tub")]
    HotTub,

    /// hotdog: \uf80f
    ///
    /// https://fontawesome.com/v5.15/icons/hotdog?style=solid
    #[strum(serialize = "fa-hotdog")]
    Hotdog,

    /// hotel: \uf594
    ///
    /// https://fontawesome.com/v5.15/icons/hotel?style=solid
    #[strum(serialize = "fa-hotel")]
    Hotel,

    /// hourglass: \uf254
    ///
    /// https://fontawesome.com/v5.15/icons/hourglass?style=solid
    #[strum(serialize = "fa-hourglass")]
    Hourglass,

    /// hourglass-end: \uf253
    ///
    /// https://fontawesome.com/v5.15/icons/hourglass-end?style=solid
    #[strum(serialize = "fa-hourglass-end")]
    HourglassEnd,

    /// hourglass-half: \uf252
    ///
    /// https://fontawesome.com/v5.15/icons/hourglass-half?style=solid
    #[strum(serialize = "fa-hourglass-half")]
    HourglassHalf,

    /// hourglass-start: \uf251
    ///
    /// https://fontawesome.com/v5.15/icons/hourglass-start?style=solid
    #[strum(serialize = "fa-hourglass-start")]
    HourglassStart,

    /// house-damage: \uf6f1
    ///
    /// https://fontawesome.com/v5.15/icons/house-damage?style=solid
    #[strum(serialize = "fa-house-damage")]
    HouseDamage,

    /// house-user: \ue065
    ///
    /// https://fontawesome.com/v5.15/icons/house-user?style=solid
    #[strum(serialize = "fa-house-user")]
    HouseUser,

    /// hryvnia: \uf6f2
    ///
    /// https://fontawesome.com/v5.15/icons/hryvnia?style=solid
    #[strum(serialize = "fa-hryvnia")]
    Hryvnia,

    /// i-cursor: \uf246
    ///
    /// https://fontawesome.com/v5.15/icons/i-cursor?style=solid
    #[strum(serialize = "fa-i-cursor")]
    ICursor,

    /// ice-cream: \uf810
    ///
    /// https://fontawesome.com/v5.15/icons/ice-cream?style=solid
    #[strum(serialize = "fa-ice-cream")]
    IceCream,

    /// icicles: \uf7ad
    ///
    /// https://fontawesome.com/v5.15/icons/icicles?style=solid
    #[strum(serialize = "fa-icicles")]
    Icicles,

    /// icons: \uf86d
    ///
    /// https://fontawesome.com/v5.15/icons/icons?style=solid
    #[strum(serialize = "fa-icons")]
    Icons,

    /// id-badge: \uf2c1
    ///
    /// https://fontawesome.com/v5.15/icons/id-badge?style=solid
    #[strum(serialize = "fa-id-badge")]
    IdBadge,

    /// id-card: \uf2c2
    ///
    /// https://fontawesome.com/v5.15/icons/id-card?style=solid
    #[strum(serialize = "fa-id-card")]
    IdCard,

    /// id-card-alt: \uf47f
    ///
    /// https://fontawesome.com/v5.15/icons/id-card-alt?style=solid
    #[strum(serialize = "fa-id-card-alt")]
    IdCardAlt,

    /// igloo: \uf7ae
    ///
    /// https://fontawesome.com/v5.15/icons/igloo?style=solid
    #[strum(serialize = "fa-igloo")]
    Igloo,

    /// image: \uf03e
    ///
    /// https://fontawesome.com/v5.15/icons/image?style=solid
    #[strum(serialize = "fa-image")]
    Image,

    /// images: \uf302
    ///
    /// https://fontawesome.com/v5.15/icons/images?style=solid
    #[strum(serialize = "fa-images")]
    Images,

    /// inbox: \uf01c
    ///
    /// https://fontawesome.com/v5.15/icons/inbox?style=solid
    #[strum(serialize = "fa-inbox")]
    Inbox,

    /// indent: \uf03c
    ///
    /// https://fontawesome.com/v5.15/icons/indent?style=solid
    #[strum(serialize = "fa-indent")]
    Indent,

    /// industry: \uf275
    ///
    /// https://fontawesome.com/v5.15/icons/industry?style=solid
    #[strum(serialize = "fa-industry")]
    Industry,

    /// infinity: \uf534
    ///
    /// https://fontawesome.com/v5.15/icons/infinity?style=solid
    #[strum(serialize = "fa-infinity")]
    Infinity,

    /// info: \uf129
    ///
    /// https://fontawesome.com/v5.15/icons/info?style=solid
    #[strum(serialize = "fa-info")]
    Info,

    /// info-circle: \uf05a
    ///
    /// https://fontawesome.com/v5.15/icons/info-circle?style=solid
    #[strum(serialize = "fa-info-circle")]
    InfoCircle,

    /// italic: \uf033
    ///
    /// https://fontawesome.com/v5.15/icons/italic?style=solid
    #[strum(serialize = "fa-italic")]
    Italic,

    /// jedi: \uf669
    ///
    /// https://fontawesome.com/v5.15/icons/jedi?style=solid
    #[strum(serialize = "fa-jedi")]
    Jedi,

    /// joint: \uf595
    ///
    /// https://fontawesome.com/v5.15/icons/joint?style=solid
    #[strum(serialize = "fa-joint")]
    Joint,

    /// journal-whills: \uf66a
    ///
    /// https://fontawesome.com/v5.15/icons/journal-whills?style=solid
    #[strum(serialize = "fa-journal-whills")]
    JournalWhills,

    /// kaaba: \uf66b
    ///
    /// https://fontawesome.com/v5.15/icons/kaaba?style=solid
    #[strum(serialize = "fa-kaaba")]
    Kaaba,

    /// key: \uf084
    ///
    /// https://fontawesome.com/v5.15/icons/key?style=solid
    #[strum(serialize = "fa-key")]
    Key,

    /// keyboard: \uf11c
    ///
    /// https://fontawesome.com/v5.15/icons/keyboard?style=solid
    #[strum(serialize = "fa-keyboard")]
    Keyboard,

    /// khanda: \uf66d
    ///
    /// https://fontawesome.com/v5.15/icons/khanda?style=solid
    #[strum(serialize = "fa-khanda")]
    Khanda,

    /// kiss: \uf596
    ///
    /// https://fontawesome.com/v5.15/icons/kiss?style=solid
    #[strum(serialize = "fa-kiss")]
    Kiss,

    /// kiss-beam: \uf597
    ///
    /// https://fontawesome.com/v5.15/icons/kiss-beam?style=solid
    #[strum(serialize = "fa-kiss-beam")]
    KissBeam,

    /// kiss-wink-heart: \uf598
    ///
    /// https://fontawesome.com/v5.15/icons/kiss-wink-heart?style=solid
    #[strum(serialize = "fa-kiss-wink-heart")]
    KissWinkHeart,

    /// kiwi-bird: \uf535
    ///
    /// https://fontawesome.com/v5.15/icons/kiwi-bird?style=solid
    #[strum(serialize = "fa-kiwi-bird")]
    KiwiBird,

    /// landmark: \uf66f
    ///
    /// https://fontawesome.com/v5.15/icons/landmark?style=solid
    #[strum(serialize = "fa-landmark")]
    Landmark,

    /// language: \uf1ab
    ///
    /// https://fontawesome.com/v5.15/icons/language?style=solid
    #[strum(serialize = "fa-language")]
    Language,

    /// laptop: \uf109
    ///
    /// https://fontawesome.com/v5.15/icons/laptop?style=solid
    #[strum(serialize = "fa-laptop")]
    Laptop,

    /// laptop-code: \uf5fc
    ///
    /// https://fontawesome.com/v5.15/icons/laptop-code?style=solid
    #[strum(serialize = "fa-laptop-code")]
    LaptopCode,

    /// laptop-house: \ue066
    ///
    /// https://fontawesome.com/v5.15/icons/laptop-house?style=solid
    #[strum(serialize = "fa-laptop-house")]
    LaptopHouse,

    /// laptop-medical: \uf812
    ///
    /// https://fontawesome.com/v5.15/icons/laptop-medical?style=solid
    #[strum(serialize = "fa-laptop-medical")]
    LaptopMedical,

    /// laugh: \uf599
    ///
    /// https://fontawesome.com/v5.15/icons/laugh?style=solid
    #[strum(serialize = "fa-laugh")]
    Laugh,

    /// laugh-beam: \uf59a
    ///
    /// https://fontawesome.com/v5.15/icons/laugh-beam?style=solid
    #[strum(serialize = "fa-laugh-beam")]
    LaughBeam,

    /// laugh-squint: \uf59b
    ///
    /// https://fontawesome.com/v5.15/icons/laugh-squint?style=solid
    #[strum(serialize = "fa-laugh-squint")]
    LaughSquint,

    /// laugh-wink: \uf59c
    ///
    /// https://fontawesome.com/v5.15/icons/laugh-wink?style=solid
    #[strum(serialize = "fa-laugh-wink")]
    LaughWink,

    /// layer-group: \uf5fd
    ///
    /// https://fontawesome.com/v5.15/icons/layer-group?style=solid
    #[strum(serialize = "fa-layer-group")]
    LayerGroup,

    /// leaf: \uf06c
    ///
    /// https://fontawesome.com/v5.15/icons/leaf?style=solid
    #[strum(serialize = "fa-leaf")]
    Leaf,

    /// lemon: \uf094
    ///
    /// https://fontawesome.com/v5.15/icons/lemon?style=solid
    #[strum(serialize = "fa-lemon")]
    Lemon,

    /// less-than: \uf536
    ///
    /// https://fontawesome.com/v5.15/icons/less-than?style=solid
    #[strum(serialize = "fa-less-than")]
    LessThan,

    /// less-than-equal: \uf537
    ///
    /// https://fontawesome.com/v5.15/icons/less-than-equal?style=solid
    #[strum(serialize = "fa-less-than-equal")]
    LessThanEqual,

    /// level-down-alt: \uf3be
    ///
    /// https://fontawesome.com/v5.15/icons/level-down-alt?style=solid
    #[strum(serialize = "fa-level-down-alt")]
    LevelDownAlt,

    /// level-up-alt: \uf3bf
    ///
    /// https://fontawesome.com/v5.15/icons/level-up-alt?style=solid
    #[strum(serialize = "fa-level-up-alt")]
    LevelUpAlt,

    /// life-ring: \uf1cd
    ///
    /// https://fontawesome.com/v5.15/icons/life-ring?style=solid
    #[strum(serialize = "fa-life-ring")]
    LifeRing,

    /// lightbulb: \uf0eb
    ///
    /// https://fontawesome.com/v5.15/icons/lightbulb?style=solid
    #[strum(serialize = "fa-lightbulb")]
    Lightbulb,

    /// link: \uf0c1
    ///
    /// https://fontawesome.com/v5.15/icons/link?style=solid
    #[strum(serialize = "fa-link")]
    Link,

    /// lira-sign: \uf195
    ///
    /// https://fontawesome.com/v5.15/icons/lira-sign?style=solid
    #[strum(serialize = "fa-lira-sign")]
    LiraSign,

    /// list: \uf03a
    ///
    /// https://fontawesome.com/v5.15/icons/list?style=solid
    #[strum(serialize = "fa-list")]
    List,

    /// list-alt: \uf022
    ///
    /// https://fontawesome.com/v5.15/icons/list-alt?style=solid
    #[strum(serialize = "fa-list-alt")]
    ListAlt,

    /// list-ol: \uf0cb
    ///
    /// https://fontawesome.com/v5.15/icons/list-ol?style=solid
    #[strum(serialize = "fa-list-ol")]
    ListOl,

    /// list-ul: \uf0ca
    ///
    /// https://fontawesome.com/v5.15/icons/list-ul?style=solid
    #[strum(serialize = "fa-list-ul")]
    ListUl,

    /// location-arrow: \uf124
    ///
    /// https://fontawesome.com/v5.15/icons/location-arrow?style=solid
    #[strum(serialize = "fa-location-arrow")]
    LocationArrow,

    /// lock: \uf023
    ///
    /// https://fontawesome.com/v5.15/icons/lock?style=solid
    #[strum(serialize = "fa-lock")]
    Lock,

    /// lock-open: \uf3c1
    ///
    /// https://fontawesome.com/v5.15/icons/lock-open?style=solid
    #[strum(serialize = "fa-lock-open")]
    LockOpen,

    /// long-arrow-alt-down: \uf309
    ///
    /// https://fontawesome.com/v5.15/icons/long-arrow-alt-down?style=solid
    #[strum(serialize = "fa-long-arrow-alt-down")]
    LongArrowAltDown,

    /// long-arrow-alt-left: \uf30a
    ///
    /// https://fontawesome.com/v5.15/icons/long-arrow-alt-left?style=solid
    #[strum(serialize = "fa-long-arrow-alt-left")]
    LongArrowAltLeft,

    /// long-arrow-alt-right: \uf30b
    ///
    /// https://fontawesome.com/v5.15/icons/long-arrow-alt-right?style=solid
    #[strum(serialize = "fa-long-arrow-alt-right")]
    LongArrowAltRight,

    /// long-arrow-alt-up: \uf30c
    ///
    /// https://fontawesome.com/v5.15/icons/long-arrow-alt-up?style=solid
    #[strum(serialize = "fa-long-arrow-alt-up")]
    LongArrowAltUp,

    /// low-vision: \uf2a8
    ///
    /// https://fontawesome.com/v5.15/icons/low-vision?style=solid
    #[strum(serialize = "fa-low-vision")]
    LowVision,

    /// luggage-cart: \uf59d
    ///
    /// https://fontawesome.com/v5.15/icons/luggage-cart?style=solid
    #[strum(serialize = "fa-luggage-cart")]
    LuggageCart,

    /// lungs: \uf604
    ///
    /// https://fontawesome.com/v5.15/icons/lungs?style=solid
    #[strum(serialize = "fa-lungs")]
    Lungs,

    /// lungs-virus: \ue067
    ///
    /// https://fontawesome.com/v5.15/icons/lungs-virus?style=solid
    #[strum(serialize = "fa-lungs-virus")]
    LungsVirus,

    /// magic: \uf0d0
    ///
    /// https://fontawesome.com/v5.15/icons/magic?style=solid
    #[strum(serialize = "fa-magic")]
    Magic,

    /// magnet: \uf076
    ///
    /// https://fontawesome.com/v5.15/icons/magnet?style=solid
    #[strum(serialize = "fa-magnet")]
    Magnet,

    /// mail-bulk: \uf674
    ///
    /// https://fontawesome.com/v5.15/icons/mail-bulk?style=solid
    #[strum(serialize = "fa-mail-bulk")]
    MailBulk,

    /// male: \uf183
    ///
    /// https://fontawesome.com/v5.15/icons/male?style=solid
    #[strum(serialize = "fa-male")]
    Male,

    /// map: \uf279
    ///
    /// https://fontawesome.com/v5.15/icons/map?style=solid
    #[strum(serialize = "fa-map")]
    Map,

    /// map-marked: \uf59f
    ///
    /// https://fontawesome.com/v5.15/icons/map-marked?style=solid
    #[strum(serialize = "fa-map-marked")]
    MapMarked,

    /// map-marked-alt: \uf5a0
    ///
    /// https://fontawesome.com/v5.15/icons/map-marked-alt?style=solid
    #[strum(serialize = "fa-map-marked-alt")]
    MapMarkedAlt,

    /// map-marker: \uf041
    ///
    /// https://fontawesome.com/v5.15/icons/map-marker?style=solid
    #[strum(serialize = "fa-map-marker")]
    MapMarker,

    /// map-marker-alt: \uf3c5
    ///
    /// https://fontawesome.com/v5.15/icons/map-marker-alt?style=solid
    #[strum(serialize = "fa-map-marker-alt")]
    MapMarkerAlt,

    /// map-pin: \uf276
    ///
    /// https://fontawesome.com/v5.15/icons/map-pin?style=solid
    #[strum(serialize = "fa-map-pin")]
    MapPin,

    /// map-signs: \uf277
    ///
    /// https://fontawesome.com/v5.15/icons/map-signs?style=solid
    #[strum(serialize = "fa-map-signs")]
    MapSigns,

    /// marker: \uf5a1
    ///
    /// https://fontawesome.com/v5.15/icons/marker?style=solid
    #[strum(serialize = "fa-marker")]
    Marker,

    /// mars: \uf222
    ///
    /// https://fontawesome.com/v5.15/icons/mars?style=solid
    #[strum(serialize = "fa-mars")]
    Mars,

    /// mars-double: \uf227
    ///
    /// https://fontawesome.com/v5.15/icons/mars-double?style=solid
    #[strum(serialize = "fa-mars-double")]
    MarsDouble,

    /// mars-stroke: \uf229
    ///
    /// https://fontawesome.com/v5.15/icons/mars-stroke?style=solid
    #[strum(serialize = "fa-mars-stroke")]
    MarsStroke,

    /// mars-stroke-h: \uf22b
    ///
    /// https://fontawesome.com/v5.15/icons/mars-stroke-h?style=solid
    #[strum(serialize = "fa-mars-stroke-h")]
    MarsStrokeH,

    /// mars-stroke-v: \uf22a
    ///
    /// https://fontawesome.com/v5.15/icons/mars-stroke-v?style=solid
    #[strum(serialize = "fa-mars-stroke-v")]
    MarsStrokeV,

    /// mask: \uf6fa
    ///
    /// https://fontawesome.com/v5.15/icons/mask?style=solid
    #[strum(serialize = "fa-mask")]
    Mask,

    /// medal: \uf5a2
    ///
    /// https://fontawesome.com/v5.15/icons/medal?style=solid
    #[strum(serialize = "fa-medal")]
    Medal,

    /// medkit: \uf0fa
    ///
    /// https://fontawesome.com/v5.15/icons/medkit?style=solid
    #[strum(serialize = "fa-medkit")]
    Medkit,

    /// meh: \uf11a
    ///
    /// https://fontawesome.com/v5.15/icons/meh?style=solid
    #[strum(serialize = "fa-meh")]
    Meh,

    /// meh-blank: \uf5a4
    ///
    /// https://fontawesome.com/v5.15/icons/meh-blank?style=solid
    #[strum(serialize = "fa-meh-blank")]
    MehBlank,

    /// meh-rolling-eyes: \uf5a5
    ///
    /// https://fontawesome.com/v5.15/icons/meh-rolling-eyes?style=solid
    #[strum(serialize = "fa-meh-rolling-eyes")]
    MehRollingEyes,

    /// memory: \uf538
    ///
    /// https://fontawesome.com/v5.15/icons/memory?style=solid
    #[strum(serialize = "fa-memory")]
    Memory,

    /// menorah: \uf676
    ///
    /// https://fontawesome.com/v5.15/icons/menorah?style=solid
    #[strum(serialize = "fa-menorah")]
    Menorah,

    /// mercury: \uf223
    ///
    /// https://fontawesome.com/v5.15/icons/mercury?style=solid
    #[strum(serialize = "fa-mercury")]
    Mercury,

    /// meteor: \uf753
    ///
    /// https://fontawesome.com/v5.15/icons/meteor?style=solid
    #[strum(serialize = "fa-meteor")]
    Meteor,

    /// microchip: \uf2db
    ///
    /// https://fontawesome.com/v5.15/icons/microchip?style=solid
    #[strum(serialize = "fa-microchip")]
    Microchip,

    /// microphone: \uf130
    ///
    /// https://fontawesome.com/v5.15/icons/microphone?style=solid
    #[strum(serialize = "fa-microphone")]
    Microphone,

    /// microphone-alt: \uf3c9
    ///
    /// https://fontawesome.com/v5.15/icons/microphone-alt?style=solid
    #[strum(serialize = "fa-microphone-alt")]
    MicrophoneAlt,

    /// microphone-alt-slash: \uf539
    ///
    /// https://fontawesome.com/v5.15/icons/microphone-alt-slash?style=solid
    #[strum(serialize = "fa-microphone-alt-slash")]
    MicrophoneAltSlash,

    /// microphone-slash: \uf131
    ///
    /// https://fontawesome.com/v5.15/icons/microphone-slash?style=solid
    #[strum(serialize = "fa-microphone-slash")]
    MicrophoneSlash,

    /// microscope: \uf610
    ///
    /// https://fontawesome.com/v5.15/icons/microscope?style=solid
    #[strum(serialize = "fa-microscope")]
    Microscope,

    /// minus: \uf068
    ///
    /// https://fontawesome.com/v5.15/icons/minus?style=solid
    #[strum(serialize = "fa-minus")]
    Minus,

    /// minus-circle: \uf056
    ///
    /// https://fontawesome.com/v5.15/icons/minus-circle?style=solid
    #[strum(serialize = "fa-minus-circle")]
    MinusCircle,

    /// minus-square: \uf146
    ///
    /// https://fontawesome.com/v5.15/icons/minus-square?style=solid
    #[strum(serialize = "fa-minus-square")]
    MinusSquare,

    /// mitten: \uf7b5
    ///
    /// https://fontawesome.com/v5.15/icons/mitten?style=solid
    #[strum(serialize = "fa-mitten")]
    Mitten,

    /// mobile: \uf10b
    ///
    /// https://fontawesome.com/v5.15/icons/mobile?style=solid
    #[strum(serialize = "fa-mobile")]
    Mobile,

    /// mobile-alt: \uf3cd
    ///
    /// https://fontawesome.com/v5.15/icons/mobile-alt?style=solid
    #[strum(serialize = "fa-mobile-alt")]
    MobileAlt,

    /// money-bill: \uf0d6
    ///
    /// https://fontawesome.com/v5.15/icons/money-bill?style=solid
    #[strum(serialize = "fa-money-bill")]
    MoneyBill,

    /// money-bill-alt: \uf3d1
    ///
    /// https://fontawesome.com/v5.15/icons/money-bill-alt?style=solid
    #[strum(serialize = "fa-money-bill-alt")]
    MoneyBillAlt,

    /// money-bill-wave: \uf53a
    ///
    /// https://fontawesome.com/v5.15/icons/money-bill-wave?style=solid
    #[strum(serialize = "fa-money-bill-wave")]
    MoneyBillWave,

    /// money-bill-wave-alt: \uf53b
    ///
    /// https://fontawesome.com/v5.15/icons/money-bill-wave-alt?style=solid
    #[strum(serialize = "fa-money-bill-wave-alt")]
    MoneyBillWaveAlt,

    /// money-check: \uf53c
    ///
    /// https://fontawesome.com/v5.15/icons/money-check?style=solid
    #[strum(serialize = "fa-money-check")]
    MoneyCheck,

    /// money-check-alt: \uf53d
    ///
    /// https://fontawesome.com/v5.15/icons/money-check-alt?style=solid
    #[strum(serialize = "fa-money-check-alt")]
    MoneyCheckAlt,

    /// monument: \uf5a6
    ///
    /// https://fontawesome.com/v5.15/icons/monument?style=solid
    #[strum(serialize = "fa-monument")]
    Monument,

    /// moon: \uf186
    ///
    /// https://fontawesome.com/v5.15/icons/moon?style=solid
    #[strum(serialize = "fa-moon")]
    Moon,

    /// mortar-pestle: \uf5a7
    ///
    /// https://fontawesome.com/v5.15/icons/mortar-pestle?style=solid
    #[strum(serialize = "fa-mortar-pestle")]
    MortarPestle,

    /// mosque: \uf678
    ///
    /// https://fontawesome.com/v5.15/icons/mosque?style=solid
    #[strum(serialize = "fa-mosque")]
    Mosque,

    /// motorcycle: \uf21c
    ///
    /// https://fontawesome.com/v5.15/icons/motorcycle?style=solid
    #[strum(serialize = "fa-motorcycle")]
    Motorcycle,

    /// mountain: \uf6fc
    ///
    /// https://fontawesome.com/v5.15/icons/mountain?style=solid
    #[strum(serialize = "fa-mountain")]
    Mountain,

    /// mouse: \uf8cc
    ///
    /// https://fontawesome.com/v5.15/icons/mouse?style=solid
    #[strum(serialize = "fa-mouse")]
    Mouse,

    /// mouse-pointer: \uf245
    ///
    /// https://fontawesome.com/v5.15/icons/mouse-pointer?style=solid
    #[strum(serialize = "fa-mouse-pointer")]
    MousePointer,

    /// mug-hot: \uf7b6
    ///
    /// https://fontawesome.com/v5.15/icons/mug-hot?style=solid
    #[strum(serialize = "fa-mug-hot")]
    MugHot,

    /// music: \uf001
    ///
    /// https://fontawesome.com/v5.15/icons/music?style=solid
    #[strum(serialize = "fa-music")]
    Music,

    /// network-wired: \uf6ff
    ///
    /// https://fontawesome.com/v5.15/icons/network-wired?style=solid
    #[strum(serialize = "fa-network-wired")]
    NetworkWired,

    /// neuter: \uf22c
    ///
    /// https://fontawesome.com/v5.15/icons/neuter?style=solid
    #[strum(serialize = "fa-neuter")]
    Neuter,

    /// newspaper: \uf1ea
    ///
    /// https://fontawesome.com/v5.15/icons/newspaper?style=solid
    #[strum(serialize = "fa-newspaper")]
    Newspaper,

    /// not-equal: \uf53e
    ///
    /// https://fontawesome.com/v5.15/icons/not-equal?style=solid
    #[strum(serialize = "fa-not-equal")]
    NotEqual,

    /// notes-medical: \uf481
    ///
    /// https://fontawesome.com/v5.15/icons/notes-medical?style=solid
    #[strum(serialize = "fa-notes-medical")]
    NotesMedical,

    /// object-group: \uf247
    ///
    /// https://fontawesome.com/v5.15/icons/object-group?style=solid
    #[strum(serialize = "fa-object-group")]
    ObjectGroup,

    /// object-ungroup: \uf248
    ///
    /// https://fontawesome.com/v5.15/icons/object-ungroup?style=solid
    #[strum(serialize = "fa-object-ungroup")]
    ObjectUngroup,

    /// oil-can: \uf613
    ///
    /// https://fontawesome.com/v5.15/icons/oil-can?style=solid
    #[strum(serialize = "fa-oil-can")]
    OilCan,

    /// om: \uf679
    ///
    /// https://fontawesome.com/v5.15/icons/om?style=solid
    #[strum(serialize = "fa-om")]
    Om,

    /// otter: \uf700
    ///
    /// https://fontawesome.com/v5.15/icons/otter?style=solid
    #[strum(serialize = "fa-otter")]
    Otter,

    /// outdent: \uf03b
    ///
    /// https://fontawesome.com/v5.15/icons/outdent?style=solid
    #[strum(serialize = "fa-outdent")]
    Outdent,

    /// pager: \uf815
    ///
    /// https://fontawesome.com/v5.15/icons/pager?style=solid
    #[strum(serialize = "fa-pager")]
    Pager,

    /// paint-brush: \uf1fc
    ///
    /// https://fontawesome.com/v5.15/icons/paint-brush?style=solid
    #[strum(serialize = "fa-paint-brush")]
    PaintBrush,

    /// paint-roller: \uf5aa
    ///
    /// https://fontawesome.com/v5.15/icons/paint-roller?style=solid
    #[strum(serialize = "fa-paint-roller")]
    PaintRoller,

    /// palette: \uf53f
    ///
    /// https://fontawesome.com/v5.15/icons/palette?style=solid
    #[strum(serialize = "fa-palette")]
    Palette,

    /// pallet: \uf482
    ///
    /// https://fontawesome.com/v5.15/icons/pallet?style=solid
    #[strum(serialize = "fa-pallet")]
    Pallet,

    /// paper-plane: \uf1d8
    ///
    /// https://fontawesome.com/v5.15/icons/paper-plane?style=solid
    #[strum(serialize = "fa-paper-plane")]
    PaperPlane,

    /// paperclip: \uf0c6
    ///
    /// https://fontawesome.com/v5.15/icons/paperclip?style=solid
    #[strum(serialize = "fa-paperclip")]
    Paperclip,

    /// parachute-box: \uf4cd
    ///
    /// https://fontawesome.com/v5.15/icons/parachute-box?style=solid
    #[strum(serialize = "fa-parachute-box")]
    ParachuteBox,

    /// paragraph: \uf1dd
    ///
    /// https://fontawesome.com/v5.15/icons/paragraph?style=solid
    #[strum(serialize = "fa-paragraph")]
    Paragraph,

    /// parking: \uf540
    ///
    /// https://fontawesome.com/v5.15/icons/parking?style=solid
    #[strum(serialize = "fa-parking")]
    Parking,

    /// passport: \uf5ab
    ///
    /// https://fontawesome.com/v5.15/icons/passport?style=solid
    #[strum(serialize = "fa-passport")]
    Passport,

    /// pastafarianism: \uf67b
    ///
    /// https://fontawesome.com/v5.15/icons/pastafarianism?style=solid
    #[strum(serialize = "fa-pastafarianism")]
    Pastafarianism,

    /// paste: \uf0ea
    ///
    /// https://fontawesome.com/v5.15/icons/paste?style=solid
    #[strum(serialize = "fa-paste")]
    Paste,

    /// pause: \uf04c
    ///
    /// https://fontawesome.com/v5.15/icons/pause?style=solid
    #[strum(serialize = "fa-pause")]
    Pause,

    /// pause-circle: \uf28b
    ///
    /// https://fontawesome.com/v5.15/icons/pause-circle?style=solid
    #[strum(serialize = "fa-pause-circle")]
    PauseCircle,

    /// paw: \uf1b0
    ///
    /// https://fontawesome.com/v5.15/icons/paw?style=solid
    #[strum(serialize = "fa-paw")]
    Paw,

    /// peace: \uf67c
    ///
    /// https://fontawesome.com/v5.15/icons/peace?style=solid
    #[strum(serialize = "fa-peace")]
    Peace,

    /// pen: \uf304
    ///
    /// https://fontawesome.com/v5.15/icons/pen?style=solid
    #[strum(serialize = "fa-pen")]
    Pen,

    /// pen-alt: \uf305
    ///
    /// https://fontawesome.com/v5.15/icons/pen-alt?style=solid
    #[strum(serialize = "fa-pen-alt")]
    PenAlt,

    /// pen-fancy: \uf5ac
    ///
    /// https://fontawesome.com/v5.15/icons/pen-fancy?style=solid
    #[strum(serialize = "fa-pen-fancy")]
    PenFancy,

    /// pen-nib: \uf5ad
    ///
    /// https://fontawesome.com/v5.15/icons/pen-nib?style=solid
    #[strum(serialize = "fa-pen-nib")]
    PenNib,

    /// pen-square: \uf14b
    ///
    /// https://fontawesome.com/v5.15/icons/pen-square?style=solid
    #[strum(serialize = "fa-pen-square")]
    PenSquare,

    /// pencil-alt: \uf303
    ///
    /// https://fontawesome.com/v5.15/icons/pencil-alt?style=solid
    #[strum(serialize = "fa-pencil-alt")]
    PencilAlt,

    /// pencil-ruler: \uf5ae
    ///
    /// https://fontawesome.com/v5.15/icons/pencil-ruler?style=solid
    #[strum(serialize = "fa-pencil-ruler")]
    PencilRuler,

    /// people-arrows: \ue068
    ///
    /// https://fontawesome.com/v5.15/icons/people-arrows?style=solid
    #[strum(serialize = "fa-people-arrows")]
    PeopleArrows,

    /// people-carry: \uf4ce
    ///
    /// https://fontawesome.com/v5.15/icons/people-carry?style=solid
    #[strum(serialize = "fa-people-carry")]
    PeopleCarry,

    /// pepper-hot: \uf816
    ///
    /// https://fontawesome.com/v5.15/icons/pepper-hot?style=solid
    #[strum(serialize = "fa-pepper-hot")]
    PepperHot,

    /// percent: \uf295
    ///
    /// https://fontawesome.com/v5.15/icons/percent?style=solid
    #[strum(serialize = "fa-percent")]
    Percent,

    /// percentage: \uf541
    ///
    /// https://fontawesome.com/v5.15/icons/percentage?style=solid
    #[strum(serialize = "fa-percentage")]
    Percentage,

    /// person-booth: \uf756
    ///
    /// https://fontawesome.com/v5.15/icons/person-booth?style=solid
    #[strum(serialize = "fa-person-booth")]
    PersonBooth,

    /// phone: \uf095
    ///
    /// https://fontawesome.com/v5.15/icons/phone?style=solid
    #[strum(serialize = "fa-phone")]
    Phone,

    /// phone-alt: \uf879
    ///
    /// https://fontawesome.com/v5.15/icons/phone-alt?style=solid
    #[strum(serialize = "fa-phone-alt")]
    PhoneAlt,

    /// phone-slash: \uf3dd
    ///
    /// https://fontawesome.com/v5.15/icons/phone-slash?style=solid
    #[strum(serialize = "fa-phone-slash")]
    PhoneSlash,

    /// phone-square: \uf098
    ///
    /// https://fontawesome.com/v5.15/icons/phone-square?style=solid
    #[strum(serialize = "fa-phone-square")]
    PhoneSquare,

    /// phone-square-alt: \uf87b
    ///
    /// https://fontawesome.com/v5.15/icons/phone-square-alt?style=solid
    #[strum(serialize = "fa-phone-square-alt")]
    PhoneSquareAlt,

    /// phone-volume: \uf2a0
    ///
    /// https://fontawesome.com/v5.15/icons/phone-volume?style=solid
    #[strum(serialize = "fa-phone-volume")]
    PhoneVolume,

    /// photo-video: \uf87c
    ///
    /// https://fontawesome.com/v5.15/icons/photo-video?style=solid
    #[strum(serialize = "fa-photo-video")]
    PhotoVideo,

    /// piggy-bank: \uf4d3
    ///
    /// https://fontawesome.com/v5.15/icons/piggy-bank?style=solid
    #[strum(serialize = "fa-piggy-bank")]
    PiggyBank,

    /// pills: \uf484
    ///
    /// https://fontawesome.com/v5.15/icons/pills?style=solid
    #[strum(serialize = "fa-pills")]
    Pills,

    /// pizza-slice: \uf818
    ///
    /// https://fontawesome.com/v5.15/icons/pizza-slice?style=solid
    #[strum(serialize = "fa-pizza-slice")]
    PizzaSlice,

    /// place-of-worship: \uf67f
    ///
    /// https://fontawesome.com/v5.15/icons/place-of-worship?style=solid
    #[strum(serialize = "fa-place-of-worship")]
    PlaceOfWorship,

    /// plane: \uf072
    ///
    /// https://fontawesome.com/v5.15/icons/plane?style=solid
    #[strum(serialize = "fa-plane")]
    Plane,

    /// plane-arrival: \uf5af
    ///
    /// https://fontawesome.com/v5.15/icons/plane-arrival?style=solid
    #[strum(serialize = "fa-plane-arrival")]
    PlaneArrival,

    /// plane-departure: \uf5b0
    ///
    /// https://fontawesome.com/v5.15/icons/plane-departure?style=solid
    #[strum(serialize = "fa-plane-departure")]
    PlaneDeparture,

    /// plane-slash: \ue069
    ///
    /// https://fontawesome.com/v5.15/icons/plane-slash?style=solid
    #[strum(serialize = "fa-plane-slash")]
    PlaneSlash,

    /// play: \uf04b
    ///
    /// https://fontawesome.com/v5.15/icons/play?style=solid
    #[strum(serialize = "fa-play")]
    Play,

    /// play-circle: \uf144
    ///
    /// https://fontawesome.com/v5.15/icons/play-circle?style=solid
    #[strum(serialize = "fa-play-circle")]
    PlayCircle,

    /// plug: \uf1e6
    ///
    /// https://fontawesome.com/v5.15/icons/plug?style=solid
    #[strum(serialize = "fa-plug")]
    Plug,

    /// plus: \uf067
    ///
    /// https://fontawesome.com/v5.15/icons/plus?style=solid
    #[strum(serialize = "fa-plus")]
    Plus,

    /// plus-circle: \uf055
    ///
    /// https://fontawesome.com/v5.15/icons/plus-circle?style=solid
    #[strum(serialize = "fa-plus-circle")]
    PlusCircle,

    /// plus-square: \uf0fe
    ///
    /// https://fontawesome.com/v5.15/icons/plus-square?style=solid
    #[strum(serialize = "fa-plus-square")]
    PlusSquare,

    /// podcast: \uf2ce
    ///
    /// https://fontawesome.com/v5.15/icons/podcast?style=solid
    #[strum(serialize = "fa-podcast")]
    Podcast,

    /// poll: \uf681
    ///
    /// https://fontawesome.com/v5.15/icons/poll?style=solid
    #[strum(serialize = "fa-poll")]
    Poll,

    /// poll-h: \uf682
    ///
    /// https://fontawesome.com/v5.15/icons/poll-h?style=solid
    #[strum(serialize = "fa-poll-h")]
    PollH,

    /// poo: \uf2fe
    ///
    /// https://fontawesome.com/v5.15/icons/poo?style=solid
    #[strum(serialize = "fa-poo")]
    Poo,

    /// poo-storm: \uf75a
    ///
    /// https://fontawesome.com/v5.15/icons/poo-storm?style=solid
    #[strum(serialize = "fa-poo-storm")]
    PooStorm,

    /// poop: \uf619
    ///
    /// https://fontawesome.com/v5.15/icons/poop?style=solid
    #[strum(serialize = "fa-poop")]
    Poop,

    /// portrait: \uf3e0
    ///
    /// https://fontawesome.com/v5.15/icons/portrait?style=solid
    #[strum(serialize = "fa-portrait")]
    Portrait,

    /// pound-sign: \uf154
    ///
    /// https://fontawesome.com/v5.15/icons/pound-sign?style=solid
    #[strum(serialize = "fa-pound-sign")]
    PoundSign,

    /// power-off: \uf011
    ///
    /// https://fontawesome.com/v5.15/icons/power-off?style=solid
    #[strum(serialize = "fa-power-off")]
    PowerOff,

    /// pray: \uf683
    ///
    /// https://fontawesome.com/v5.15/icons/pray?style=solid
    #[strum(serialize = "fa-pray")]
    Pray,

    /// praying-hands: \uf684
    ///
    /// https://fontawesome.com/v5.15/icons/praying-hands?style=solid
    #[strum(serialize = "fa-praying-hands")]
    PrayingHands,

    /// prescription: \uf5b1
    ///
    /// https://fontawesome.com/v5.15/icons/prescription?style=solid
    #[strum(serialize = "fa-prescription")]
    Prescription,

    /// prescription-bottle: \uf485
    ///
    /// https://fontawesome.com/v5.15/icons/prescription-bottle?style=solid
    #[strum(serialize = "fa-prescription-bottle")]
    PrescriptionBottle,

    /// prescription-bottle-alt: \uf486
    ///
    /// https://fontawesome.com/v5.15/icons/prescription-bottle-alt?style=solid
    #[strum(serialize = "fa-prescription-bottle-alt")]
    PrescriptionBottleAlt,

    /// print: \uf02f
    ///
    /// https://fontawesome.com/v5.15/icons/print?style=solid
    #[strum(serialize = "fa-print")]
    Print,

    /// procedures: \uf487
    ///
    /// https://fontawesome.com/v5.15/icons/procedures?style=solid
    #[strum(serialize = "fa-procedures")]
    Procedures,

    /// project-diagram: \uf542
    ///
    /// https://fontawesome.com/v5.15/icons/project-diagram?style=solid
    #[strum(serialize = "fa-project-diagram")]
    ProjectDiagram,

    /// pump-medical: \ue06a
    ///
    /// https://fontawesome.com/v5.15/icons/pump-medical?style=solid
    #[strum(serialize = "fa-pump-medical")]
    PumpMedical,

    /// pump-soap: \ue06b
    ///
    /// https://fontawesome.com/v5.15/icons/pump-soap?style=solid
    #[strum(serialize = "fa-pump-soap")]
    PumpSoap,

    /// puzzle-piece: \uf12e
    ///
    /// https://fontawesome.com/v5.15/icons/puzzle-piece?style=solid
    #[strum(serialize = "fa-puzzle-piece")]
    PuzzlePiece,

    /// qrcode: \uf029
    ///
    /// https://fontawesome.com/v5.15/icons/qrcode?style=solid
    #[strum(serialize = "fa-qrcode")]
    Qrcode,

    /// question: \uf128
    ///
    /// https://fontawesome.com/v5.15/icons/question?style=solid
    #[strum(serialize = "fa-question")]
    Question,

    /// question-circle: \uf059
    ///
    /// https://fontawesome.com/v5.15/icons/question-circle?style=solid
    #[strum(serialize = "fa-question-circle")]
    QuestionCircle,

    /// quidditch: \uf458
    ///
    /// https://fontawesome.com/v5.15/icons/quidditch?style=solid
    #[strum(serialize = "fa-quidditch")]
    Quidditch,

    /// quote-left: \uf10d
    ///
    /// https://fontawesome.com/v5.15/icons/quote-left?style=solid
    #[strum(serialize = "fa-quote-left")]
    QuoteLeft,

    /// quote-right: \uf10e
    ///
    /// https://fontawesome.com/v5.15/icons/quote-right?style=solid
    #[strum(serialize = "fa-quote-right")]
    QuoteRight,

    /// quran: \uf687
    ///
    /// https://fontawesome.com/v5.15/icons/quran?style=solid
    #[strum(serialize = "fa-quran")]
    Quran,

    /// radiation: \uf7b9
    ///
    /// https://fontawesome.com/v5.15/icons/radiation?style=solid
    #[strum(serialize = "fa-radiation")]
    Radiation,

    /// radiation-alt: \uf7ba
    ///
    /// https://fontawesome.com/v5.15/icons/radiation-alt?style=solid
    #[strum(serialize = "fa-radiation-alt")]
    RadiationAlt,

    /// rainbow: \uf75b
    ///
    /// https://fontawesome.com/v5.15/icons/rainbow?style=solid
    #[strum(serialize = "fa-rainbow")]
    Rainbow,

    /// random: \uf074
    ///
    /// https://fontawesome.com/v5.15/icons/random?style=solid
    #[strum(serialize = "fa-random")]
    Random,

    /// receipt: \uf543
    ///
    /// https://fontawesome.com/v5.15/icons/receipt?style=solid
    #[strum(serialize = "fa-receipt")]
    Receipt,

    /// record-vinyl: \uf8d9
    ///
    /// https://fontawesome.com/v5.15/icons/record-vinyl?style=solid
    #[strum(serialize = "fa-record-vinyl")]
    RecordVinyl,

    /// recycle: \uf1b8
    ///
    /// https://fontawesome.com/v5.15/icons/recycle?style=solid
    #[strum(serialize = "fa-recycle")]
    Recycle,

    /// redo: \uf01e
    ///
    /// https://fontawesome.com/v5.15/icons/redo?style=solid
    #[strum(serialize = "fa-redo")]
    Redo,

    /// redo-alt: \uf2f9
    ///
    /// https://fontawesome.com/v5.15/icons/redo-alt?style=solid
    #[strum(serialize = "fa-redo-alt")]
    RedoAlt,

    /// registered: \uf25d
    ///
    /// https://fontawesome.com/v5.15/icons/registered?style=solid
    #[strum(serialize = "fa-registered")]
    Registered,

    /// remove-format: \uf87d
    ///
    /// https://fontawesome.com/v5.15/icons/remove-format?style=solid
    #[strum(serialize = "fa-remove-format")]
    RemoveFormat,

    /// reply: \uf3e5
    ///
    /// https://fontawesome.com/v5.15/icons/reply?style=solid
    #[strum(serialize = "fa-reply")]
    Reply,

    /// reply-all: \uf122
    ///
    /// https://fontawesome.com/v5.15/icons/reply-all?style=solid
    #[strum(serialize = "fa-reply-all")]
    ReplyAll,

    /// republican: \uf75e
    ///
    /// https://fontawesome.com/v5.15/icons/republican?style=solid
    #[strum(serialize = "fa-republican")]
    Republican,

    /// restroom: \uf7bd
    ///
    /// https://fontawesome.com/v5.15/icons/restroom?style=solid
    #[strum(serialize = "fa-restroom")]
    Restroom,

    /// retweet: \uf079
    ///
    /// https://fontawesome.com/v5.15/icons/retweet?style=solid
    #[strum(serialize = "fa-retweet")]
    Retweet,

    /// ribbon: \uf4d6
    ///
    /// https://fontawesome.com/v5.15/icons/ribbon?style=solid
    #[strum(serialize = "fa-ribbon")]
    Ribbon,

    /// ring: \uf70b
    ///
    /// https://fontawesome.com/v5.15/icons/ring?style=solid
    #[strum(serialize = "fa-ring")]
    Ring,

    /// road: \uf018
    ///
    /// https://fontawesome.com/v5.15/icons/road?style=solid
    #[strum(serialize = "fa-road")]
    Road,

    /// robot: \uf544
    ///
    /// https://fontawesome.com/v5.15/icons/robot?style=solid
    #[strum(serialize = "fa-robot")]
    Robot,

    /// rocket: \uf135
    ///
    /// https://fontawesome.com/v5.15/icons/rocket?style=solid
    #[strum(serialize = "fa-rocket")]
    Rocket,

    /// route: \uf4d7
    ///
    /// https://fontawesome.com/v5.15/icons/route?style=solid
    #[strum(serialize = "fa-route")]
    Route,

    /// rss: \uf09e
    ///
    /// https://fontawesome.com/v5.15/icons/rss?style=solid
    #[strum(serialize = "fa-rss")]
    Rss,

    /// rss-square: \uf143
    ///
    /// https://fontawesome.com/v5.15/icons/rss-square?style=solid
    #[strum(serialize = "fa-rss-square")]
    RssSquare,

    /// ruble-sign: \uf158
    ///
    /// https://fontawesome.com/v5.15/icons/ruble-sign?style=solid
    #[strum(serialize = "fa-ruble-sign")]
    RubleSign,

    /// ruler: \uf545
    ///
    /// https://fontawesome.com/v5.15/icons/ruler?style=solid
    #[strum(serialize = "fa-ruler")]
    Ruler,

    /// ruler-combined: \uf546
    ///
    /// https://fontawesome.com/v5.15/icons/ruler-combined?style=solid
    #[strum(serialize = "fa-ruler-combined")]
    RulerCombined,

    /// ruler-horizontal: \uf547
    ///
    /// https://fontawesome.com/v5.15/icons/ruler-horizontal?style=solid
    #[strum(serialize = "fa-ruler-horizontal")]
    RulerHorizontal,

    /// ruler-vertical: \uf548
    ///
    /// https://fontawesome.com/v5.15/icons/ruler-vertical?style=solid
    #[strum(serialize = "fa-ruler-vertical")]
    RulerVertical,

    /// running: \uf70c
    ///
    /// https://fontawesome.com/v5.15/icons/running?style=solid
    #[strum(serialize = "fa-running")]
    Running,

    /// rupee-sign: \uf156
    ///
    /// https://fontawesome.com/v5.15/icons/rupee-sign?style=solid
    #[strum(serialize = "fa-rupee-sign")]
    RupeeSign,

    /// sad-cry: \uf5b3
    ///
    /// https://fontawesome.com/v5.15/icons/sad-cry?style=solid
    #[strum(serialize = "fa-sad-cry")]
    SadCry,

    /// sad-tear: \uf5b4
    ///
    /// https://fontawesome.com/v5.15/icons/sad-tear?style=solid
    #[strum(serialize = "fa-sad-tear")]
    SadTear,

    /// satellite: \uf7bf
    ///
    /// https://fontawesome.com/v5.15/icons/satellite?style=solid
    #[strum(serialize = "fa-satellite")]
    Satellite,

    /// satellite-dish: \uf7c0
    ///
    /// https://fontawesome.com/v5.15/icons/satellite-dish?style=solid
    #[strum(serialize = "fa-satellite-dish")]
    SatelliteDish,

    /// save: \uf0c7
    ///
    /// https://fontawesome.com/v5.15/icons/save?style=solid
    #[strum(serialize = "fa-save")]
    Save,

    /// school: \uf549
    ///
    /// https://fontawesome.com/v5.15/icons/school?style=solid
    #[strum(serialize = "fa-school")]
    School,

    /// screwdriver: \uf54a
    ///
    /// https://fontawesome.com/v5.15/icons/screwdriver?style=solid
    #[strum(serialize = "fa-screwdriver")]
    Screwdriver,

    /// scroll: \uf70e
    ///
    /// https://fontawesome.com/v5.15/icons/scroll?style=solid
    #[strum(serialize = "fa-scroll")]
    Scroll,

    /// sd-card: \uf7c2
    ///
    /// https://fontawesome.com/v5.15/icons/sd-card?style=solid
    #[strum(serialize = "fa-sd-card")]
    SdCard,

    /// search: \uf002
    ///
    /// https://fontawesome.com/v5.15/icons/search?style=solid
    #[strum(serialize = "fa-search")]
    Search,

    /// search-dollar: \uf688
    ///
    /// https://fontawesome.com/v5.15/icons/search-dollar?style=solid
    #[strum(serialize = "fa-search-dollar")]
    SearchDollar,

    /// search-location: \uf689
    ///
    /// https://fontawesome.com/v5.15/icons/search-location?style=solid
    #[strum(serialize = "fa-search-location")]
    SearchLocation,

    /// search-minus: \uf010
    ///
    /// https://fontawesome.com/v5.15/icons/search-minus?style=solid
    #[strum(serialize = "fa-search-minus")]
    SearchMinus,

    /// search-plus: \uf00e
    ///
    /// https://fontawesome.com/v5.15/icons/search-plus?style=solid
    #[strum(serialize = "fa-search-plus")]
    SearchPlus,

    /// seedling: \uf4d8
    ///
    /// https://fontawesome.com/v5.15/icons/seedling?style=solid
    #[strum(serialize = "fa-seedling")]
    Seedling,

    /// server: \uf233
    ///
    /// https://fontawesome.com/v5.15/icons/server?style=solid
    #[strum(serialize = "fa-server")]
    Server,

    /// shapes: \uf61f
    ///
    /// https://fontawesome.com/v5.15/icons/shapes?style=solid
    #[strum(serialize = "fa-shapes")]
    Shapes,

    /// share: \uf064
    ///
    /// https://fontawesome.com/v5.15/icons/share?style=solid
    #[strum(serialize = "fa-share")]
    Share,

    /// share-alt: \uf1e0
    ///
    /// https://fontawesome.com/v5.15/icons/share-alt?style=solid
    #[strum(serialize = "fa-share-alt")]
    ShareAlt,

    /// share-alt-square: \uf1e1
    ///
    /// https://fontawesome.com/v5.15/icons/share-alt-square?style=solid
    #[strum(serialize = "fa-share-alt-square")]
    ShareAltSquare,

    /// share-square: \uf14d
    ///
    /// https://fontawesome.com/v5.15/icons/share-square?style=solid
    #[strum(serialize = "fa-share-square")]
    ShareSquare,

    /// shekel-sign: \uf20b
    ///
    /// https://fontawesome.com/v5.15/icons/shekel-sign?style=solid
    #[strum(serialize = "fa-shekel-sign")]
    ShekelSign,

    /// shield-alt: \uf3ed
    ///
    /// https://fontawesome.com/v5.15/icons/shield-alt?style=solid
    #[strum(serialize = "fa-shield-alt")]
    ShieldAlt,

    /// shield-virus: \ue06c
    ///
    /// https://fontawesome.com/v5.15/icons/shield-virus?style=solid
    #[strum(serialize = "fa-shield-virus")]
    ShieldVirus,

    /// ship: \uf21a
    ///
    /// https://fontawesome.com/v5.15/icons/ship?style=solid
    #[strum(serialize = "fa-ship")]
    Ship,

    /// shipping-fast: \uf48b
    ///
    /// https://fontawesome.com/v5.15/icons/shipping-fast?style=solid
    #[strum(serialize = "fa-shipping-fast")]
    ShippingFast,

    /// shoe-prints: \uf54b
    ///
    /// https://fontawesome.com/v5.15/icons/shoe-prints?style=solid
    #[strum(serialize = "fa-shoe-prints")]
    ShoePrints,

    /// shopping-bag: \uf290
    ///
    /// https://fontawesome.com/v5.15/icons/shopping-bag?style=solid
    #[strum(serialize = "fa-shopping-bag")]
    ShoppingBag,

    /// shopping-basket: \uf291
    ///
    /// https://fontawesome.com/v5.15/icons/shopping-basket?style=solid
    #[strum(serialize = "fa-shopping-basket")]
    ShoppingBasket,

    /// shopping-cart: \uf07a
    ///
    /// https://fontawesome.com/v5.15/icons/shopping-cart?style=solid
    #[strum(serialize = "fa-shopping-cart")]
    ShoppingCart,

    /// shower: \uf2cc
    ///
    /// https://fontawesome.com/v5.15/icons/shower?style=solid
    #[strum(serialize = "fa-shower")]
    Shower,

    /// shuttle-van: \uf5b6
    ///
    /// https://fontawesome.com/v5.15/icons/shuttle-van?style=solid
    #[strum(serialize = "fa-shuttle-van")]
    ShuttleVan,

    /// sign: \uf4d9
    ///
    /// https://fontawesome.com/v5.15/icons/sign?style=solid
    #[strum(serialize = "fa-sign")]
    Sign,

    /// sign-in-alt: \uf2f6
    ///
    /// https://fontawesome.com/v5.15/icons/sign-in-alt?style=solid
    #[strum(serialize = "fa-sign-in-alt")]
    SignInAlt,

    /// sign-language: \uf2a7
    ///
    /// https://fontawesome.com/v5.15/icons/sign-language?style=solid
    #[strum(serialize = "fa-sign-language")]
    SignLanguage,

    /// sign-out-alt: \uf2f5
    ///
    /// https://fontawesome.com/v5.15/icons/sign-out-alt?style=solid
    #[strum(serialize = "fa-sign-out-alt")]
    SignOutAlt,

    /// signal: \uf012
    ///
    /// https://fontawesome.com/v5.15/icons/signal?style=solid
    #[strum(serialize = "fa-signal")]
    Signal,

    /// signature: \uf5b7
    ///
    /// https://fontawesome.com/v5.15/icons/signature?style=solid
    #[strum(serialize = "fa-signature")]
    Signature,

    /// sim-card: \uf7c4
    ///
    /// https://fontawesome.com/v5.15/icons/sim-card?style=solid
    #[strum(serialize = "fa-sim-card")]
    SimCard,

    /// sink: \ue06d
    ///
    /// https://fontawesome.com/v5.15/icons/sink?style=solid
    #[strum(serialize = "fa-sink")]
    Sink,

    /// sitemap: \uf0e8
    ///
    /// https://fontawesome.com/v5.15/icons/sitemap?style=solid
    #[strum(serialize = "fa-sitemap")]
    Sitemap,

    /// skating: \uf7c5
    ///
    /// https://fontawesome.com/v5.15/icons/skating?style=solid
    #[strum(serialize = "fa-skating")]
    Skating,

    /// skiing: \uf7c9
    ///
    /// https://fontawesome.com/v5.15/icons/skiing?style=solid
    #[strum(serialize = "fa-skiing")]
    Skiing,

    /// skiing-nordic: \uf7ca
    ///
    /// https://fontawesome.com/v5.15/icons/skiing-nordic?style=solid
    #[strum(serialize = "fa-skiing-nordic")]
    SkiingNordic,

    /// skull: \uf54c
    ///
    /// https://fontawesome.com/v5.15/icons/skull?style=solid
    #[strum(serialize = "fa-skull")]
    Skull,

    /// skull-crossbones: \uf714
    ///
    /// https://fontawesome.com/v5.15/icons/skull-crossbones?style=solid
    #[strum(serialize = "fa-skull-crossbones")]
    SkullCrossbones,

    /// slash: \uf715
    ///
    /// https://fontawesome.com/v5.15/icons/slash?style=solid
    #[strum(serialize = "fa-slash")]
    Slash,

    /// sleigh: \uf7cc
    ///
    /// https://fontawesome.com/v5.15/icons/sleigh?style=solid
    #[strum(serialize = "fa-sleigh")]
    Sleigh,

    /// sliders-h: \uf1de
    ///
    /// https://fontawesome.com/v5.15/icons/sliders-h?style=solid
    #[strum(serialize = "fa-sliders-h")]
    SlidersH,

    /// smile: \uf118
    ///
    /// https://fontawesome.com/v5.15/icons/smile?style=solid
    #[strum(serialize = "fa-smile")]
    Smile,

    /// smile-beam: \uf5b8
    ///
    /// https://fontawesome.com/v5.15/icons/smile-beam?style=solid
    #[strum(serialize = "fa-smile-beam")]
    SmileBeam,

    /// smile-wink: \uf4da
    ///
    /// https://fontawesome.com/v5.15/icons/smile-wink?style=solid
    #[strum(serialize = "fa-smile-wink")]
    SmileWink,

    /// smog: \uf75f
    ///
    /// https://fontawesome.com/v5.15/icons/smog?style=solid
    #[strum(serialize = "fa-smog")]
    Smog,

    /// smoking: \uf48d
    ///
    /// https://fontawesome.com/v5.15/icons/smoking?style=solid
    #[strum(serialize = "fa-smoking")]
    Smoking,

    /// smoking-ban: \uf54d
    ///
    /// https://fontawesome.com/v5.15/icons/smoking-ban?style=solid
    #[strum(serialize = "fa-smoking-ban")]
    SmokingBan,

    /// sms: \uf7cd
    ///
    /// https://fontawesome.com/v5.15/icons/sms?style=solid
    #[strum(serialize = "fa-sms")]
    Sms,

    /// snowboarding: \uf7ce
    ///
    /// https://fontawesome.com/v5.15/icons/snowboarding?style=solid
    #[strum(serialize = "fa-snowboarding")]
    Snowboarding,

    /// snowflake: \uf2dc
    ///
    /// https://fontawesome.com/v5.15/icons/snowflake?style=solid
    #[strum(serialize = "fa-snowflake")]
    Snowflake,

    /// snowman: \uf7d0
    ///
    /// https://fontawesome.com/v5.15/icons/snowman?style=solid
    #[strum(serialize = "fa-snowman")]
    Snowman,

    /// snowplow: \uf7d2
    ///
    /// https://fontawesome.com/v5.15/icons/snowplow?style=solid
    #[strum(serialize = "fa-snowplow")]
    Snowplow,

    /// soap: \ue06e
    ///
    /// https://fontawesome.com/v5.15/icons/soap?style=solid
    #[strum(serialize = "fa-soap")]
    Soap,

    /// socks: \uf696
    ///
    /// https://fontawesome.com/v5.15/icons/socks?style=solid
    #[strum(serialize = "fa-socks")]
    Socks,

    /// solar-panel: \uf5ba
    ///
    /// https://fontawesome.com/v5.15/icons/solar-panel?style=solid
    #[strum(serialize = "fa-solar-panel")]
    SolarPanel,

    /// sort: \uf0dc
    ///
    /// https://fontawesome.com/v5.15/icons/sort?style=solid
    #[strum(serialize = "fa-sort")]
    Sort,

    /// sort-alpha-down: \uf15d
    ///
    /// https://fontawesome.com/v5.15/icons/sort-alpha-down?style=solid
    #[strum(serialize = "fa-sort-alpha-down")]
    SortAlphaDown,

    /// sort-alpha-down-alt: \uf881
    ///
    /// https://fontawesome.com/v5.15/icons/sort-alpha-down-alt?style=solid
    #[strum(serialize = "fa-sort-alpha-down-alt")]
    SortAlphaDownAlt,

    /// sort-alpha-up: \uf15e
    ///
    /// https://fontawesome.com/v5.15/icons/sort-alpha-up?style=solid
    #[strum(serialize = "fa-sort-alpha-up")]
    SortAlphaUp,

    /// sort-alpha-up-alt: \uf882
    ///
    /// https://fontawesome.com/v5.15/icons/sort-alpha-up-alt?style=solid
    #[strum(serialize = "fa-sort-alpha-up-alt")]
    SortAlphaUpAlt,

    /// sort-amount-down: \uf160
    ///
    /// https://fontawesome.com/v5.15/icons/sort-amount-down?style=solid
    #[strum(serialize = "fa-sort-amount-down")]
    SortAmountDown,

    /// sort-amount-down-alt: \uf884
    ///
    /// https://fontawesome.com/v5.15/icons/sort-amount-down-alt?style=solid
    #[strum(serialize = "fa-sort-amount-down-alt")]
    SortAmountDownAlt,

    /// sort-amount-up: \uf161
    ///
    /// https://fontawesome.com/v5.15/icons/sort-amount-up?style=solid
    #[strum(serialize = "fa-sort-amount-up")]
    SortAmountUp,

    /// sort-amount-up-alt: \uf885
    ///
    /// https://fontawesome.com/v5.15/icons/sort-amount-up-alt?style=solid
    #[strum(serialize = "fa-sort-amount-up-alt")]
    SortAmountUpAlt,

    /// sort-down: \uf0dd
    ///
    /// https://fontawesome.com/v5.15/icons/sort-down?style=solid
    #[strum(serialize = "fa-sort-down")]
    SortDown,

    /// sort-numeric-down: \uf162
    ///
    /// https://fontawesome.com/v5.15/icons/sort-numeric-down?style=solid
    #[strum(serialize = "fa-sort-numeric-down")]
    SortNumericDown,

    /// sort-numeric-down-alt: \uf886
    ///
    /// https://fontawesome.com/v5.15/icons/sort-numeric-down-alt?style=solid
    #[strum(serialize = "fa-sort-numeric-down-alt")]
    SortNumericDownAlt,

    /// sort-numeric-up: \uf163
    ///
    /// https://fontawesome.com/v5.15/icons/sort-numeric-up?style=solid
    #[strum(serialize = "fa-sort-numeric-up")]
    SortNumericUp,

    /// sort-numeric-up-alt: \uf887
    ///
    /// https://fontawesome.com/v5.15/icons/sort-numeric-up-alt?style=solid
    #[strum(serialize = "fa-sort-numeric-up-alt")]
    SortNumericUpAlt,

    /// sort-up: \uf0de
    ///
    /// https://fontawesome.com/v5.15/icons/sort-up?style=solid
    #[strum(serialize = "fa-sort-up")]
    SortUp,

    /// spa: \uf5bb
    ///
    /// https://fontawesome.com/v5.15/icons/spa?style=solid
    #[strum(serialize = "fa-spa")]
    Spa,

    /// space-shuttle: \uf197
    ///
    /// https://fontawesome.com/v5.15/icons/space-shuttle?style=solid
    #[strum(serialize = "fa-space-shuttle")]
    SpaceShuttle,

    /// spell-check: \uf891
    ///
    /// https://fontawesome.com/v5.15/icons/spell-check?style=solid
    #[strum(serialize = "fa-spell-check")]
    SpellCheck,

    /// spider: \uf717
    ///
    /// https://fontawesome.com/v5.15/icons/spider?style=solid
    #[strum(serialize = "fa-spider")]
    Spider,

    /// spinner: \uf110
    ///
    /// https://fontawesome.com/v5.15/icons/spinner?style=solid
    #[strum(serialize = "fa-spinner")]
    Spinner,

    /// splotch: \uf5bc
    ///
    /// https://fontawesome.com/v5.15/icons/splotch?style=solid
    #[strum(serialize = "fa-splotch")]
    Splotch,

    /// spray-can: \uf5bd
    ///
    /// https://fontawesome.com/v5.15/icons/spray-can?style=solid
    #[strum(serialize = "fa-spray-can")]
    SprayCan,

    /// square: \uf0c8
    ///
    /// https://fontawesome.com/v5.15/icons/square?style=solid
    #[strum(serialize = "fa-square")]
    Square,

    /// square-full: \uf45c
    ///
    /// https://fontawesome.com/v5.15/icons/square-full?style=solid
    #[strum(serialize = "fa-square-full")]
    SquareFull,

    /// square-root-alt: \uf698
    ///
    /// https://fontawesome.com/v5.15/icons/square-root-alt?style=solid
    #[strum(serialize = "fa-square-root-alt")]
    SquareRootAlt,

    /// stamp: \uf5bf
    ///
    /// https://fontawesome.com/v5.15/icons/stamp?style=solid
    #[strum(serialize = "fa-stamp")]
    Stamp,

    /// star: \uf005
    ///
    /// https://fontawesome.com/v5.15/icons/star?style=solid
    #[strum(serialize = "fa-star")]
    Star,

    /// star-and-crescent: \uf699
    ///
    /// https://fontawesome.com/v5.15/icons/star-and-crescent?style=solid
    #[strum(serialize = "fa-star-and-crescent")]
    StarAndCrescent,

    /// star-half: \uf089
    ///
    /// https://fontawesome.com/v5.15/icons/star-half?style=solid
    #[strum(serialize = "fa-star-half")]
    StarHalf,

    /// star-half-alt: \uf5c0
    ///
    /// https://fontawesome.com/v5.15/icons/star-half-alt?style=solid
    #[strum(serialize = "fa-star-half-alt")]
    StarHalfAlt,

    /// star-of-david: \uf69a
    ///
    /// https://fontawesome.com/v5.15/icons/star-of-david?style=solid
    #[strum(serialize = "fa-star-of-david")]
    StarOfDavid,

    /// star-of-life: \uf621
    ///
    /// https://fontawesome.com/v5.15/icons/star-of-life?style=solid
    #[strum(serialize = "fa-star-of-life")]
    StarOfLife,

    /// step-backward: \uf048
    ///
    /// https://fontawesome.com/v5.15/icons/step-backward?style=solid
    #[strum(serialize = "fa-step-backward")]
    StepBackward,

    /// step-forward: \uf051
    ///
    /// https://fontawesome.com/v5.15/icons/step-forward?style=solid
    #[strum(serialize = "fa-step-forward")]
    StepForward,

    /// stethoscope: \uf0f1
    ///
    /// https://fontawesome.com/v5.15/icons/stethoscope?style=solid
    #[strum(serialize = "fa-stethoscope")]
    Stethoscope,

    /// sticky-note: \uf249
    ///
    /// https://fontawesome.com/v5.15/icons/sticky-note?style=solid
    #[strum(serialize = "fa-sticky-note")]
    StickyNote,

    /// stop: \uf04d
    ///
    /// https://fontawesome.com/v5.15/icons/stop?style=solid
    #[strum(serialize = "fa-stop")]
    Stop,

    /// stop-circle: \uf28d
    ///
    /// https://fontawesome.com/v5.15/icons/stop-circle?style=solid
    #[strum(serialize = "fa-stop-circle")]
    StopCircle,

    /// stopwatch: \uf2f2
    ///
    /// https://fontawesome.com/v5.15/icons/stopwatch?style=solid
    #[strum(serialize = "fa-stopwatch")]
    Stopwatch,

    /// stopwatch-20: \ue06f
    ///
    /// https://fontawesome.com/v5.15/icons/stopwatch-20?style=solid
    #[strum(serialize = "fa-stopwatch-20")]
    Stopwatch20,

    /// store: \uf54e
    ///
    /// https://fontawesome.com/v5.15/icons/store?style=solid
    #[strum(serialize = "fa-store")]
    Store,

    /// store-alt: \uf54f
    ///
    /// https://fontawesome.com/v5.15/icons/store-alt?style=solid
    #[strum(serialize = "fa-store-alt")]
    StoreAlt,

    /// store-alt-slash: \ue070
    ///
    /// https://fontawesome.com/v5.15/icons/store-alt-slash?style=solid
    #[strum(serialize = "fa-store-alt-slash")]
    StoreAltSlash,

    /// store-slash: \ue071
    ///
    /// https://fontawesome.com/v5.15/icons/store-slash?style=solid
    #[strum(serialize = "fa-store-slash")]
    StoreSlash,

    /// stream: \uf550
    ///
    /// https://fontawesome.com/v5.15/icons/stream?style=solid
    #[strum(serialize = "fa-stream")]
    Stream,

    /// street-view: \uf21d
    ///
    /// https://fontawesome.com/v5.15/icons/street-view?style=solid
    #[strum(serialize = "fa-street-view")]
    StreetView,

    /// strikethrough: \uf0cc
    ///
    /// https://fontawesome.com/v5.15/icons/strikethrough?style=solid
    #[strum(serialize = "fa-strikethrough")]
    Strikethrough,

    /// stroopwafel: \uf551
    ///
    /// https://fontawesome.com/v5.15/icons/stroopwafel?style=solid
    #[strum(serialize = "fa-stroopwafel")]
    Stroopwafel,

    /// subscript: \uf12c
    ///
    /// https://fontawesome.com/v5.15/icons/subscript?style=solid
    #[strum(serialize = "fa-subscript")]
    Subscript,

    /// subway: \uf239
    ///
    /// https://fontawesome.com/v5.15/icons/subway?style=solid
    #[strum(serialize = "fa-subway")]
    Subway,

    /// suitcase: \uf0f2
    ///
    /// https://fontawesome.com/v5.15/icons/suitcase?style=solid
    #[strum(serialize = "fa-suitcase")]
    Suitcase,

    /// suitcase-rolling: \uf5c1
    ///
    /// https://fontawesome.com/v5.15/icons/suitcase-rolling?style=solid
    #[strum(serialize = "fa-suitcase-rolling")]
    SuitcaseRolling,

    /// sun: \uf185
    ///
    /// https://fontawesome.com/v5.15/icons/sun?style=solid
    #[strum(serialize = "fa-sun")]
    Sun,

    /// superscript: \uf12b
    ///
    /// https://fontawesome.com/v5.15/icons/superscript?style=solid
    #[strum(serialize = "fa-superscript")]
    Superscript,

    /// surprise: \uf5c2
    ///
    /// https://fontawesome.com/v5.15/icons/surprise?style=solid
    #[strum(serialize = "fa-surprise")]
    Surprise,

    /// swatchbook: \uf5c3
    ///
    /// https://fontawesome.com/v5.15/icons/swatchbook?style=solid
    #[strum(serialize = "fa-swatchbook")]
    Swatchbook,

    /// swimmer: \uf5c4
    ///
    /// https://fontawesome.com/v5.15/icons/swimmer?style=solid
    #[strum(serialize = "fa-swimmer")]
    Swimmer,

    /// swimming-pool: \uf5c5
    ///
    /// https://fontawesome.com/v5.15/icons/swimming-pool?style=solid
    #[strum(serialize = "fa-swimming-pool")]
    SwimmingPool,

    /// synagogue: \uf69b
    ///
    /// https://fontawesome.com/v5.15/icons/synagogue?style=solid
    #[strum(serialize = "fa-synagogue")]
    Synagogue,

    /// sync: \uf021
    ///
    /// https://fontawesome.com/v5.15/icons/sync?style=solid
    #[strum(serialize = "fa-sync")]
    Sync,

    /// sync-alt: \uf2f1
    ///
    /// https://fontawesome.com/v5.15/icons/sync-alt?style=solid
    #[strum(serialize = "fa-sync-alt")]
    SyncAlt,

    /// syringe: \uf48e
    ///
    /// https://fontawesome.com/v5.15/icons/syringe?style=solid
    #[strum(serialize = "fa-syringe")]
    Syringe,

    /// table: \uf0ce
    ///
    /// https://fontawesome.com/v5.15/icons/table?style=solid
    #[strum(serialize = "fa-table")]
    Table,

    /// table-tennis: \uf45d
    ///
    /// https://fontawesome.com/v5.15/icons/table-tennis?style=solid
    #[strum(serialize = "fa-table-tennis")]
    TableTennis,

    /// tablet: \uf10a
    ///
    /// https://fontawesome.com/v5.15/icons/tablet?style=solid
    #[strum(serialize = "fa-tablet")]
    Tablet,

    /// tablet-alt: \uf3fa
    ///
    /// https://fontawesome.com/v5.15/icons/tablet-alt?style=solid
    #[strum(serialize = "fa-tablet-alt")]
    TabletAlt,

    /// tablets: \uf490
    ///
    /// https://fontawesome.com/v5.15/icons/tablets?style=solid
    #[strum(serialize = "fa-tablets")]
    Tablets,

    /// tachometer-alt: \uf3fd
    ///
    /// https://fontawesome.com/v5.15/icons/tachometer-alt?style=solid
    #[strum(serialize = "fa-tachometer-alt")]
    TachometerAlt,

    /// tag: \uf02b
    ///
    /// https://fontawesome.com/v5.15/icons/tag?style=solid
    #[strum(serialize = "fa-tag")]
    Tag,

    /// tags: \uf02c
    ///
    /// https://fontawesome.com/v5.15/icons/tags?style=solid
    #[strum(serialize = "fa-tags")]
    Tags,

    /// tape: \uf4db
    ///
    /// https://fontawesome.com/v5.15/icons/tape?style=solid
    #[strum(serialize = "fa-tape")]
    Tape,

    /// tasks: \uf0ae
    ///
    /// https://fontawesome.com/v5.15/icons/tasks?style=solid
    #[strum(serialize = "fa-tasks")]
    Tasks,

    /// taxi: \uf1ba
    ///
    /// https://fontawesome.com/v5.15/icons/taxi?style=solid
    #[strum(serialize = "fa-taxi")]
    Taxi,

    /// teeth: \uf62e
    ///
    /// https://fontawesome.com/v5.15/icons/teeth?style=solid
    #[strum(serialize = "fa-teeth")]
    Teeth,

    /// teeth-open: \uf62f
    ///
    /// https://fontawesome.com/v5.15/icons/teeth-open?style=solid
    #[strum(serialize = "fa-teeth-open")]
    TeethOpen,

    /// temperature-high: \uf769
    ///
    /// https://fontawesome.com/v5.15/icons/temperature-high?style=solid
    #[strum(serialize = "fa-temperature-high")]
    TemperatureHigh,

    /// temperature-low: \uf76b
    ///
    /// https://fontawesome.com/v5.15/icons/temperature-low?style=solid
    #[strum(serialize = "fa-temperature-low")]
    TemperatureLow,

    /// tenge: \uf7d7
    ///
    /// https://fontawesome.com/v5.15/icons/tenge?style=solid
    #[strum(serialize = "fa-tenge")]
    Tenge,

    /// terminal: \uf120
    ///
    /// https://fontawesome.com/v5.15/icons/terminal?style=solid
    #[strum(serialize = "fa-terminal")]
    Terminal,

    /// text-height: \uf034
    ///
    /// https://fontawesome.com/v5.15/icons/text-height?style=solid
    #[strum(serialize = "fa-text-height")]
    TextHeight,

    /// text-width: \uf035
    ///
    /// https://fontawesome.com/v5.15/icons/text-width?style=solid
    #[strum(serialize = "fa-text-width")]
    TextWidth,

    /// th: \uf00a
    ///
    /// https://fontawesome.com/v5.15/icons/th?style=solid
    #[strum(serialize = "fa-th")]
    Th,

    /// th-large: \uf009
    ///
    /// https://fontawesome.com/v5.15/icons/th-large?style=solid
    #[strum(serialize = "fa-th-large")]
    ThLarge,

    /// th-list: \uf00b
    ///
    /// https://fontawesome.com/v5.15/icons/th-list?style=solid
    #[strum(serialize = "fa-th-list")]
    ThList,

    /// theater-masks: \uf630
    ///
    /// https://fontawesome.com/v5.15/icons/theater-masks?style=solid
    #[strum(serialize = "fa-theater-masks")]
    TheaterMasks,

    /// thermometer: \uf491
    ///
    /// https://fontawesome.com/v5.15/icons/thermometer?style=solid
    #[strum(serialize = "fa-thermometer")]
    Thermometer,

    /// thermometer-empty: \uf2cb
    ///
    /// https://fontawesome.com/v5.15/icons/thermometer-empty?style=solid
    #[strum(serialize = "fa-thermometer-empty")]
    ThermometerEmpty,

    /// thermometer-full: \uf2c7
    ///
    /// https://fontawesome.com/v5.15/icons/thermometer-full?style=solid
    #[strum(serialize = "fa-thermometer-full")]
    ThermometerFull,

    /// thermometer-half: \uf2c9
    ///
    /// https://fontawesome.com/v5.15/icons/thermometer-half?style=solid
    #[strum(serialize = "fa-thermometer-half")]
    ThermometerHalf,

    /// thermometer-quarter: \uf2ca
    ///
    /// https://fontawesome.com/v5.15/icons/thermometer-quarter?style=solid
    #[strum(serialize = "fa-thermometer-quarter")]
    ThermometerQuarter,

    /// thermometer-three-quarters: \uf2c8
    ///
    /// https://fontawesome.com/v5.15/icons/thermometer-three-quarters?style=solid
    #[strum(serialize = "fa-thermometer-three-quarters")]
    ThermometerThreeQuarters,

    /// thumbs-down: \uf165
    ///
    /// https://fontawesome.com/v5.15/icons/thumbs-down?style=solid
    #[strum(serialize = "fa-thumbs-down")]
    ThumbsDown,

    /// thumbs-up: \uf164
    ///
    /// https://fontawesome.com/v5.15/icons/thumbs-up?style=solid
    #[strum(serialize = "fa-thumbs-up")]
    ThumbsUp,

    /// thumbtack: \uf08d
    ///
    /// https://fontawesome.com/v5.15/icons/thumbtack?style=solid
    #[strum(serialize = "fa-thumbtack")]
    Thumbtack,

    /// ticket-alt: \uf3ff
    ///
    /// https://fontawesome.com/v5.15/icons/ticket-alt?style=solid
    #[strum(serialize = "fa-ticket-alt")]
    TicketAlt,

    /// times: \uf00d
    ///
    /// https://fontawesome.com/v5.15/icons/times?style=solid
    #[strum(serialize = "fa-times")]
    Times,

    /// times-circle: \uf057
    ///
    /// https://fontawesome.com/v5.15/icons/times-circle?style=solid
    #[strum(serialize = "fa-times-circle")]
    TimesCircle,

    /// tint: \uf043
    ///
    /// https://fontawesome.com/v5.15/icons/tint?style=solid
    #[strum(serialize = "fa-tint")]
    Tint,

    /// tint-slash: \uf5c7
    ///
    /// https://fontawesome.com/v5.15/icons/tint-slash?style=solid
    #[strum(serialize = "fa-tint-slash")]
    TintSlash,

    /// tired: \uf5c8
    ///
    /// https://fontawesome.com/v5.15/icons/tired?style=solid
    #[strum(serialize = "fa-tired")]
    Tired,

    /// toggle-off: \uf204
    ///
    /// https://fontawesome.com/v5.15/icons/toggle-off?style=solid
    #[strum(serialize = "fa-toggle-off")]
    ToggleOff,

    /// toggle-on: \uf205
    ///
    /// https://fontawesome.com/v5.15/icons/toggle-on?style=solid
    #[strum(serialize = "fa-toggle-on")]
    ToggleOn,

    /// toilet: \uf7d8
    ///
    /// https://fontawesome.com/v5.15/icons/toilet?style=solid
    #[strum(serialize = "fa-toilet")]
    Toilet,

    /// toilet-paper: \uf71e
    ///
    /// https://fontawesome.com/v5.15/icons/toilet-paper?style=solid
    #[strum(serialize = "fa-toilet-paper")]
    ToiletPaper,

    /// toilet-paper-slash: \ue072
    ///
    /// https://fontawesome.com/v5.15/icons/toilet-paper-slash?style=solid
    #[strum(serialize = "fa-toilet-paper-slash")]
    ToiletPaperSlash,

    /// toolbox: \uf552
    ///
    /// https://fontawesome.com/v5.15/icons/toolbox?style=solid
    #[strum(serialize = "fa-toolbox")]
    Toolbox,

    /// tools: \uf7d9
    ///
    /// https://fontawesome.com/v5.15/icons/tools?style=solid
    #[strum(serialize = "fa-tools")]
    Tools,

    /// tooth: \uf5c9
    ///
    /// https://fontawesome.com/v5.15/icons/tooth?style=solid
    #[strum(serialize = "fa-tooth")]
    Tooth,

    /// torah: \uf6a0
    ///
    /// https://fontawesome.com/v5.15/icons/torah?style=solid
    #[strum(serialize = "fa-torah")]
    Torah,

    /// torii-gate: \uf6a1
    ///
    /// https://fontawesome.com/v5.15/icons/torii-gate?style=solid
    #[strum(serialize = "fa-torii-gate")]
    ToriiGate,

    /// tractor: \uf722
    ///
    /// https://fontawesome.com/v5.15/icons/tractor?style=solid
    #[strum(serialize = "fa-tractor")]
    Tractor,

    /// trademark: \uf25c
    ///
    /// https://fontawesome.com/v5.15/icons/trademark?style=solid
    #[strum(serialize = "fa-trademark")]
    Trademark,

    /// traffic-light: \uf637
    ///
    /// https://fontawesome.com/v5.15/icons/traffic-light?style=solid
    #[strum(serialize = "fa-traffic-light")]
    TrafficLight,

    /// trailer: \ue041
    ///
    /// https://fontawesome.com/v5.15/icons/trailer?style=solid
    #[strum(serialize = "fa-trailer")]
    Trailer,

    /// train: \uf238
    ///
    /// https://fontawesome.com/v5.15/icons/train?style=solid
    #[strum(serialize = "fa-train")]
    Train,

    /// tram: \uf7da
    ///
    /// https://fontawesome.com/v5.15/icons/tram?style=solid
    #[strum(serialize = "fa-tram")]
    Tram,

    /// transgender: \uf224
    ///
    /// https://fontawesome.com/v5.15/icons/transgender?style=solid
    #[strum(serialize = "fa-transgender")]
    Transgender,

    /// transgender-alt: \uf225
    ///
    /// https://fontawesome.com/v5.15/icons/transgender-alt?style=solid
    #[strum(serialize = "fa-transgender-alt")]
    TransgenderAlt,

    /// trash: \uf1f8
    ///
    /// https://fontawesome.com/v5.15/icons/trash?style=solid
    #[strum(serialize = "fa-trash")]
    Trash,

    /// trash-alt: \uf2ed
    ///
    /// https://fontawesome.com/v5.15/icons/trash-alt?style=solid
    #[strum(serialize = "fa-trash-alt")]
    TrashAlt,

    /// trash-restore: \uf829
    ///
    /// https://fontawesome.com/v5.15/icons/trash-restore?style=solid
    #[strum(serialize = "fa-trash-restore")]
    TrashRestore,

    /// trash-restore-alt: \uf82a
    ///
    /// https://fontawesome.com/v5.15/icons/trash-restore-alt?style=solid
    #[strum(serialize = "fa-trash-restore-alt")]
    TrashRestoreAlt,

    /// tree: \uf1bb
    ///
    /// https://fontawesome.com/v5.15/icons/tree?style=solid
    #[strum(serialize = "fa-tree")]
    Tree,

    /// trophy: \uf091
    ///
    /// https://fontawesome.com/v5.15/icons/trophy?style=solid
    #[strum(serialize = "fa-trophy")]
    Trophy,

    /// truck: \uf0d1
    ///
    /// https://fontawesome.com/v5.15/icons/truck?style=solid
    #[strum(serialize = "fa-truck")]
    Truck,

    /// truck-loading: \uf4de
    ///
    /// https://fontawesome.com/v5.15/icons/truck-loading?style=solid
    #[strum(serialize = "fa-truck-loading")]
    TruckLoading,

    /// truck-monster: \uf63b
    ///
    /// https://fontawesome.com/v5.15/icons/truck-monster?style=solid
    #[strum(serialize = "fa-truck-monster")]
    TruckMonster,

    /// truck-moving: \uf4df
    ///
    /// https://fontawesome.com/v5.15/icons/truck-moving?style=solid
    #[strum(serialize = "fa-truck-moving")]
    TruckMoving,

    /// truck-pickup: \uf63c
    ///
    /// https://fontawesome.com/v5.15/icons/truck-pickup?style=solid
    #[strum(serialize = "fa-truck-pickup")]
    TruckPickup,

    /// tshirt: \uf553
    ///
    /// https://fontawesome.com/v5.15/icons/tshirt?style=solid
    #[strum(serialize = "fa-tshirt")]
    Tshirt,

    /// tty: \uf1e4
    ///
    /// https://fontawesome.com/v5.15/icons/tty?style=solid
    #[strum(serialize = "fa-tty")]
    Tty,

    /// tv: \uf26c
    ///
    /// https://fontawesome.com/v5.15/icons/tv?style=solid
    #[strum(serialize = "fa-tv")]
    Tv,

    /// umbrella: \uf0e9
    ///
    /// https://fontawesome.com/v5.15/icons/umbrella?style=solid
    #[strum(serialize = "fa-umbrella")]
    Umbrella,

    /// umbrella-beach: \uf5ca
    ///
    /// https://fontawesome.com/v5.15/icons/umbrella-beach?style=solid
    #[strum(serialize = "fa-umbrella-beach")]
    UmbrellaBeach,

    /// underline: \uf0cd
    ///
    /// https://fontawesome.com/v5.15/icons/underline?style=solid
    #[strum(serialize = "fa-underline")]
    Underline,

    /// undo: \uf0e2
    ///
    /// https://fontawesome.com/v5.15/icons/undo?style=solid
    #[strum(serialize = "fa-undo")]
    Undo,

    /// undo-alt: \uf2ea
    ///
    /// https://fontawesome.com/v5.15/icons/undo-alt?style=solid
    #[strum(serialize = "fa-undo-alt")]
    UndoAlt,

    /// universal-access: \uf29a
    ///
    /// https://fontawesome.com/v5.15/icons/universal-access?style=solid
    #[strum(serialize = "fa-universal-access")]
    UniversalAccess,

    /// university: \uf19c
    ///
    /// https://fontawesome.com/v5.15/icons/university?style=solid
    #[strum(serialize = "fa-university")]
    University,

    /// unlink: \uf127
    ///
    /// https://fontawesome.com/v5.15/icons/unlink?style=solid
    #[strum(serialize = "fa-unlink")]
    Unlink,

    /// unlock: \uf09c
    ///
    /// https://fontawesome.com/v5.15/icons/unlock?style=solid
    #[strum(serialize = "fa-unlock")]
    Unlock,

    /// unlock-alt: \uf13e
    ///
    /// https://fontawesome.com/v5.15/icons/unlock-alt?style=solid
    #[strum(serialize = "fa-unlock-alt")]
    UnlockAlt,

    /// upload: \uf093
    ///
    /// https://fontawesome.com/v5.15/icons/upload?style=solid
    #[strum(serialize = "fa-upload")]
    Upload,

    /// user: \uf007
    ///
    /// https://fontawesome.com/v5.15/icons/user?style=solid
    #[strum(serialize = "fa-user")]
    User,

    /// user-alt: \uf406
    ///
    /// https://fontawesome.com/v5.15/icons/user-alt?style=solid
    #[strum(serialize = "fa-user-alt")]
    UserAlt,

    /// user-alt-slash: \uf4fa
    ///
    /// https://fontawesome.com/v5.15/icons/user-alt-slash?style=solid
    #[strum(serialize = "fa-user-alt-slash")]
    UserAltSlash,

    /// user-astronaut: \uf4fb
    ///
    /// https://fontawesome.com/v5.15/icons/user-astronaut?style=solid
    #[strum(serialize = "fa-user-astronaut")]
    UserAstronaut,

    /// user-check: \uf4fc
    ///
    /// https://fontawesome.com/v5.15/icons/user-check?style=solid
    #[strum(serialize = "fa-user-check")]
    UserCheck,

    /// user-circle: \uf2bd
    ///
    /// https://fontawesome.com/v5.15/icons/user-circle?style=solid
    #[strum(serialize = "fa-user-circle")]
    UserCircle,

    /// user-clock: \uf4fd
    ///
    /// https://fontawesome.com/v5.15/icons/user-clock?style=solid
    #[strum(serialize = "fa-user-clock")]
    UserClock,

    /// user-cog: \uf4fe
    ///
    /// https://fontawesome.com/v5.15/icons/user-cog?style=solid
    #[strum(serialize = "fa-user-cog")]
    UserCog,

    /// user-edit: \uf4ff
    ///
    /// https://fontawesome.com/v5.15/icons/user-edit?style=solid
    #[strum(serialize = "fa-user-edit")]
    UserEdit,

    /// user-friends: \uf500
    ///
    /// https://fontawesome.com/v5.15/icons/user-friends?style=solid
    #[strum(serialize = "fa-user-friends")]
    UserFriends,

    /// user-graduate: \uf501
    ///
    /// https://fontawesome.com/v5.15/icons/user-graduate?style=solid
    #[strum(serialize = "fa-user-graduate")]
    UserGraduate,

    /// user-injured: \uf728
    ///
    /// https://fontawesome.com/v5.15/icons/user-injured?style=solid
    #[strum(serialize = "fa-user-injured")]
    UserInjured,

    /// user-lock: \uf502
    ///
    /// https://fontawesome.com/v5.15/icons/user-lock?style=solid
    #[strum(serialize = "fa-user-lock")]
    UserLock,

    /// user-md: \uf0f0
    ///
    /// https://fontawesome.com/v5.15/icons/user-md?style=solid
    #[strum(serialize = "fa-user-md")]
    UserMd,

    /// user-minus: \uf503
    ///
    /// https://fontawesome.com/v5.15/icons/user-minus?style=solid
    #[strum(serialize = "fa-user-minus")]
    UserMinus,

    /// user-ninja: \uf504
    ///
    /// https://fontawesome.com/v5.15/icons/user-ninja?style=solid
    #[strum(serialize = "fa-user-ninja")]
    UserNinja,

    /// user-nurse: \uf82f
    ///
    /// https://fontawesome.com/v5.15/icons/user-nurse?style=solid
    #[strum(serialize = "fa-user-nurse")]
    UserNurse,

    /// user-plus: \uf234
    ///
    /// https://fontawesome.com/v5.15/icons/user-plus?style=solid
    #[strum(serialize = "fa-user-plus")]
    UserPlus,

    /// user-secret: \uf21b
    ///
    /// https://fontawesome.com/v5.15/icons/user-secret?style=solid
    #[strum(serialize = "fa-user-secret")]
    UserSecret,

    /// user-shield: \uf505
    ///
    /// https://fontawesome.com/v5.15/icons/user-shield?style=solid
    #[strum(serialize = "fa-user-shield")]
    UserShield,

    /// user-slash: \uf506
    ///
    /// https://fontawesome.com/v5.15/icons/user-slash?style=solid
    #[strum(serialize = "fa-user-slash")]
    UserSlash,

    /// user-tag: \uf507
    ///
    /// https://fontawesome.com/v5.15/icons/user-tag?style=solid
    #[strum(serialize = "fa-user-tag")]
    UserTag,

    /// user-tie: \uf508
    ///
    /// https://fontawesome.com/v5.15/icons/user-tie?style=solid
    #[strum(serialize = "fa-user-tie")]
    UserTie,

    /// user-times: \uf235
    ///
    /// https://fontawesome.com/v5.15/icons/user-times?style=solid
    #[strum(serialize = "fa-user-times")]
    UserTimes,

    /// users: \uf0c0
    ///
    /// https://fontawesome.com/v5.15/icons/users?style=solid
    #[strum(serialize = "fa-users")]
    Users,

    /// users-cog: \uf509
    ///
    /// https://fontawesome.com/v5.15/icons/users-cog?style=solid
    #[strum(serialize = "fa-users-cog")]
    UsersCog,

    /// users-slash: \ue073
    ///
    /// https://fontawesome.com/v5.15/icons/users-slash?style=solid
    #[strum(serialize = "fa-users-slash")]
    UsersSlash,

    /// utensil-spoon: \uf2e5
    ///
    /// https://fontawesome.com/v5.15/icons/utensil-spoon?style=solid
    #[strum(serialize = "fa-utensil-spoon")]
    UtensilSpoon,

    /// utensils: \uf2e7
    ///
    /// https://fontawesome.com/v5.15/icons/utensils?style=solid
    #[strum(serialize = "fa-utensils")]
    Utensils,

    /// vector-square: \uf5cb
    ///
    /// https://fontawesome.com/v5.15/icons/vector-square?style=solid
    #[strum(serialize = "fa-vector-square")]
    VectorSquare,

    /// venus: \uf221
    ///
    /// https://fontawesome.com/v5.15/icons/venus?style=solid
    #[strum(serialize = "fa-venus")]
    Venus,

    /// venus-double: \uf226
    ///
    /// https://fontawesome.com/v5.15/icons/venus-double?style=solid
    #[strum(serialize = "fa-venus-double")]
    VenusDouble,

    /// venus-mars: \uf228
    ///
    /// https://fontawesome.com/v5.15/icons/venus-mars?style=solid
    #[strum(serialize = "fa-venus-mars")]
    VenusMars,

    /// vest: \ue085
    ///
    /// https://fontawesome.com/v5.15/icons/vest?style=solid
    #[strum(serialize = "fa-vest")]
    Vest,

    /// vest-patches: \ue086
    ///
    /// https://fontawesome.com/v5.15/icons/vest-patches?style=solid
    #[strum(serialize = "fa-vest-patches")]
    VestPatches,

    /// vial: \uf492
    ///
    /// https://fontawesome.com/v5.15/icons/vial?style=solid
    #[strum(serialize = "fa-vial")]
    Vial,

    /// vials: \uf493
    ///
    /// https://fontawesome.com/v5.15/icons/vials?style=solid
    #[strum(serialize = "fa-vials")]
    Vials,

    /// video: \uf03d
    ///
    /// https://fontawesome.com/v5.15/icons/video?style=solid
    #[strum(serialize = "fa-video")]
    Video,

    /// video-slash: \uf4e2
    ///
    /// https://fontawesome.com/v5.15/icons/video-slash?style=solid
    #[strum(serialize = "fa-video-slash")]
    VideoSlash,

    /// vihara: \uf6a7
    ///
    /// https://fontawesome.com/v5.15/icons/vihara?style=solid
    #[strum(serialize = "fa-vihara")]
    Vihara,

    /// virus: \ue074
    ///
    /// https://fontawesome.com/v5.15/icons/virus?style=solid
    #[strum(serialize = "fa-virus")]
    Virus,

    /// virus-slash: \ue075
    ///
    /// https://fontawesome.com/v5.15/icons/virus-slash?style=solid
    #[strum(serialize = "fa-virus-slash")]
    VirusSlash,

    /// viruses: \ue076
    ///
    /// https://fontawesome.com/v5.15/icons/viruses?style=solid
    #[strum(serialize = "fa-viruses")]
    Viruses,

    /// voicemail: \uf897
    ///
    /// https://fontawesome.com/v5.15/icons/voicemail?style=solid
    #[strum(serialize = "fa-voicemail")]
    Voicemail,

    /// volleyball-ball: \uf45f
    ///
    /// https://fontawesome.com/v5.15/icons/volleyball-ball?style=solid
    #[strum(serialize = "fa-volleyball-ball")]
    VolleyballBall,

    /// volume-down: \uf027
    ///
    /// https://fontawesome.com/v5.15/icons/volume-down?style=solid
    #[strum(serialize = "fa-volume-down")]
    VolumeDown,

    /// volume-mute: \uf6a9
    ///
    /// https://fontawesome.com/v5.15/icons/volume-mute?style=solid
    #[strum(serialize = "fa-volume-mute")]
    VolumeMute,

    /// volume-off: \uf026
    ///
    /// https://fontawesome.com/v5.15/icons/volume-off?style=solid
    #[strum(serialize = "fa-volume-off")]
    VolumeOff,

    /// volume-up: \uf028
    ///
    /// https://fontawesome.com/v5.15/icons/volume-up?style=solid
    #[strum(serialize = "fa-volume-up")]
    VolumeUp,

    /// vote-yea: \uf772
    ///
    /// https://fontawesome.com/v5.15/icons/vote-yea?style=solid
    #[strum(serialize = "fa-vote-yea")]
    VoteYea,

    /// vr-cardboard: \uf729
    ///
    /// https://fontawesome.com/v5.15/icons/vr-cardboard?style=solid
    #[strum(serialize = "fa-vr-cardboard")]
    VrCardboard,

    /// walking: \uf554
    ///
    /// https://fontawesome.com/v5.15/icons/walking?style=solid
    #[strum(serialize = "fa-walking")]
    Walking,

    /// wallet: \uf555
    ///
    /// https://fontawesome.com/v5.15/icons/wallet?style=solid
    #[strum(serialize = "fa-wallet")]
    Wallet,

    /// warehouse: \uf494
    ///
    /// https://fontawesome.com/v5.15/icons/warehouse?style=solid
    #[strum(serialize = "fa-warehouse")]
    Warehouse,

    /// water: \uf773
    ///
    /// https://fontawesome.com/v5.15/icons/water?style=solid
    #[strum(serialize = "fa-water")]
    Water,

    /// wave-square: \uf83e
    ///
    /// https://fontawesome.com/v5.15/icons/wave-square?style=solid
    #[strum(serialize = "fa-wave-square")]
    WaveSquare,

    /// weight: \uf496
    ///
    /// https://fontawesome.com/v5.15/icons/weight?style=solid
    #[strum(serialize = "fa-weight")]
    Weight,

    /// weight-hanging: \uf5cd
    ///
    /// https://fontawesome.com/v5.15/icons/weight-hanging?style=solid
    #[strum(serialize = "fa-weight-hanging")]
    WeightHanging,

    /// wheelchair: \uf193
    ///
    /// https://fontawesome.com/v5.15/icons/wheelchair?style=solid
    #[strum(serialize = "fa-wheelchair")]
    Wheelchair,

    /// wifi: \uf1eb
    ///
    /// https://fontawesome.com/v5.15/icons/wifi?style=solid
    #[strum(serialize = "fa-wifi")]
    Wifi,

    /// wind: \uf72e
    ///
    /// https://fontawesome.com/v5.15/icons/wind?style=solid
    #[strum(serialize = "fa-wind")]
    Wind,

    /// window-close: \uf410
    ///
    /// https://fontawesome.com/v5.15/icons/window-close?style=solid
    #[strum(serialize = "fa-window-close")]
    WindowClose,

    /// window-maximize: \uf2d0
    ///
    /// https://fontawesome.com/v5.15/icons/window-maximize?style=solid
    #[strum(serialize = "fa-window-maximize")]
    WindowMaximize,

    /// window-minimize: \uf2d1
    ///
    /// https://fontawesome.com/v5.15/icons/window-minimize?style=solid
    #[strum(serialize = "fa-window-minimize")]
    WindowMinimize,

    /// window-restore: \uf2d2
    ///
    /// https://fontawesome.com/v5.15/icons/window-restore?style=solid
    #[strum(serialize = "fa-window-restore")]
    WindowRestore,

    /// wine-bottle: \uf72f
    ///
    /// https://fontawesome.com/v5.15/icons/wine-bottle?style=solid
    #[strum(serialize = "fa-wine-bottle")]
    WineBottle,

    /// wine-glass: \uf4e3
    ///
    /// https://fontawesome.com/v5.15/icons/wine-glass?style=solid
    #[strum(serialize = "fa-wine-glass")]
    WineGlass,

    /// wine-glass-alt: \uf5ce
    ///
    /// https://fontawesome.com/v5.15/icons/wine-glass-alt?style=solid
    #[strum(serialize = "fa-wine-glass-alt")]
    WineGlassAlt,

    /// won-sign: \uf159
    ///
    /// https://fontawesome.com/v5.15/icons/won-sign?style=solid
    #[strum(serialize = "fa-won-sign")]
    WonSign,

    /// wrench: \uf0ad
    ///
    /// https://fontawesome.com/v5.15/icons/wrench?style=solid
    #[strum(serialize = "fa-wrench")]
    Wrench,

    /// x-ray: \uf497
    ///
    /// https://fontawesome.com/v5.15/icons/x-ray?style=solid
    #[strum(serialize = "fa-x-ray")]
    XRay,

    /// yen-sign: \uf157
    ///
    /// https://fontawesome.com/v5.15/icons/yen-sign?style=solid
    #[strum(serialize = "fa-yen-sign")]
    YenSign,

    /// yin-yang: \uf6ad
    ///
    /// https://fontawesome.com/v5.15/icons/yin-yang?style=solid
    #[strum(serialize = "fa-yin-yang")]
    YinYang,
}
