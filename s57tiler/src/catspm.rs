use crate::geojson_builder::JsonObject;
use serde_json::Value;

/// CATSPM
///
/// Expected input:
///     ID	Meaning	INT 1	S-4
///     1	firing danger area mark	IQ 50,125	441.2;
///     2	target mark	IQ 51;
///     3	marker ship mark	IQ 52;
///     4	degaussing range mark	IQ 54;	448.3;
///     5	barge mark	IQ 53;
///     6	cable mark	IQ 55, 123;	443.6; 458;
///     7	spoil ground mark	IQ 56;	446.3;
///     8	outfall mark	IQ 57;	444.4;
///     9	ODAS (Ocean-Data-Acquisition-System)	IQ 58;	462.9;
///     10	recording mark	IQ 59;
///     11	seaplane anchorage mark	IQ 60;
///     12	recreation zone mark	IQ 62;
///     13	private mark	IQ 70;
///     14	mooring mark	 	431.5;
///     15	LANBY (Large Automatic Navigational Buoy)	IQ 26;	474.4-5;
///     16	leading mark	IQ 120;	458;
///     17	measured distance mark	IQ 122;	458;
///     18	notice mark	IQ 126;	456.8;
///     19	TSS mark (Traffic Separation Scheme)	IQ 61;
///     20	anchoring prohibited mark
///     21	berthing prohibited mark
///     22	overtaking prohibited mark
///     23	two-way traffic prohibited mark
///     24	'reduced wake' mark
///     25	speed limit mark	 	456.2;
///     26	stop mark
///     27	general warning mark
///     28	'sound ship's siren' mark
///     29	restricted vertical clearance mark
///     30	maximum vessel's draught mark
///     31	restricted horizontal clearance mark
///     32	strong current warning mark
///     33	berthing permitted mark
///     34	overhead power cable mark
///     35	'channel edge gradient' mark
///     36	telephone mark
///     37	ferry crossing mark
///     38	marine traffic lights
///     39	pipeline mark
///     40	anchorage mark
///     41	clearing mark	IQ 121;	458;
///     42	control mark
///     43	diving mark
///     44	refuge beacon	IQ 124;
///     45	foul ground mark
///     46	yachting mark
///     47	heliport mark
///     48	GPS mark
///     49	seaplane landing mark
///     50	entry prohibited mark
///     51	work in progress mark
///     52	mark with unknown purpose
///     53	wellhead mark
///     54	channel separation mark
///     55	marine farm mark
///     56	artificial reef mark
/// Remarks:
///     A mark may be a beacon, a buoy, a signpost or may take another form.
///     Value number 38 should be encoded using object class signal station, traffic (SISTAT).
#[derive(Eq, PartialEq)]
pub enum Catspm {
    FiringDangerAreaMark,
    TargetMark,
    MarkerShipMar,
    DegaussingRangeMark,
    BargeMark,
    CableMark,
    SpoilGroundMark,
    OutfallMark,
    OceanDataAcquisitionSystem,
    RecordingMark,
    SeaplaneAnchorageMark,
    RecreationZoneMark,
    PrivateMark,
    MooringMark,
    LargeAutomaticNavigationalBuoy,
    LeadingMark,
    MeasuredDistanceMark,
    NoticeMark,
    TrafficSeparationScheme,
    AnchoringProhibitedMark,
    BerthingProhibitedMark,
    OvertakingProhibitedMark,
    TwoWayTrafficProhibitedMark,
    ReducedWakeMark,
    SpeedLimitMark,
    StopMark,
    GeneralWarningMark,
    SoundShipsSirenMark,
    RestrictedVerticalClearanceMark,
    MaximumVesselsDraughtMark,
    RestrictedHorizontalClearanceMark,
    StrongCurrentWarningMark,
    BerthingPermittedMark,
    OverheadPowerCableMark,
    ChannelEdgeGradientMark,
    TelephoneMark,
    FerryCrossingMark,
    MarineTrafficLights,
    PipelineMark,
    AnchorageMark,
    ClearingMark,
    ControlMark,
    DivingMark,
    RefugeBeacon,
    FoulGroundMark,
    YachtingMark,
    HeliportMark,
    GPSMark,
    SeaplaneLandingMark,
    EntryProhibitedMark,
    WorkInProgressMark,
    MarkWithUnknownPurpose,
    WellheadMark,
    ChannelSeparationMark,
    MarineFarmMark,
    ArtificialReefMark,
}

impl Catspm {
    pub fn from_value(properties: &JsonObject) -> Vec<Catspm> {
        properties.get("CATSPM").and_then(|value| {
            value.as_array()
        }).map(|item| {
            item.iter().map(|ea| {
                match ea {
                    Value::String(n) => {
                        n.clone()
                    },
                    _ => {
                        panic!("unexpected value type for CATSPM");
                    }
                }
            }).map(|n| {
                match n.as_str() {
                    "1" => Some(Catspm::FiringDangerAreaMark),
                    "2" => Some(Catspm::TargetMark),
                    "3" => Some(Catspm::MarkerShipMar),
                    "4" => Some(Catspm::DegaussingRangeMark),
                    "5" => Some(Catspm::BargeMark),
                    "6" => Some(Catspm::CableMark),
                    "7" => Some(Catspm::SpoilGroundMark),
                    "8" => Some(Catspm::OutfallMark),
                    "9" => Some(Catspm::OceanDataAcquisitionSystem),
                    "10" => Some(Catspm::RecordingMark),
                    "11" => Some(Catspm::SeaplaneAnchorageMark),
                    "12" => Some(Catspm::RecreationZoneMark),
                    "13" => Some(Catspm::PrivateMark),
                    "14" => Some(Catspm::MooringMark),
                    "15" => Some(Catspm::LargeAutomaticNavigationalBuoy),
                    "16" => Some(Catspm::LeadingMark),
                    "17" => Some(Catspm::MeasuredDistanceMark),
                    "18" => Some(Catspm::NoticeMark),
                    "19" => Some(Catspm::TrafficSeparationScheme),
                    "20" => Some(Catspm::AnchoringProhibitedMark),
                    "21" => Some(Catspm::BerthingProhibitedMark),
                    "22" => Some(Catspm::OvertakingProhibitedMark),
                    "23" => Some(Catspm::TwoWayTrafficProhibitedMark),
                    "24" => Some(Catspm::ReducedWakeMark),
                    "25" => Some(Catspm::SpeedLimitMark),
                    "26" => Some(Catspm::StopMark),
                    "27" => Some(Catspm::GeneralWarningMark),
                    "28" => Some(Catspm::SoundShipsSirenMark),
                    "29" => Some(Catspm::RestrictedVerticalClearanceMark),
                    "30" => Some(Catspm::MaximumVesselsDraughtMark),
                    "31" => Some(Catspm::RestrictedHorizontalClearanceMark),
                    "32" => Some(Catspm::StrongCurrentWarningMark),
                    "33" => Some(Catspm::BerthingPermittedMark),
                    "34" => Some(Catspm::OverheadPowerCableMark),
                    "35" => Some(Catspm::ChannelEdgeGradientMark),
                    "36" => Some(Catspm::TelephoneMark),
                    "37" => Some(Catspm::FerryCrossingMark),
                    "38" => Some(Catspm::MarineTrafficLights),
                    "39" => Some(Catspm::PipelineMark),
                    "40" => Some(Catspm::AnchorageMark),
                    "41" => Some(Catspm::ClearingMark),
                    "42" => Some(Catspm::ControlMark),
                    "43" => Some(Catspm::DivingMark),
                    "44" => Some(Catspm::RefugeBeacon),
                    "45" => Some(Catspm::FoulGroundMark),
                    "46" => Some(Catspm::YachtingMark),
                    "47" => Some(Catspm::HeliportMark),
                    "48" => Some(Catspm::GPSMark),
                    "49" => Some(Catspm::SeaplaneLandingMark),
                    "50" => Some(Catspm::EntryProhibitedMark),
                    "51" => Some(Catspm::WorkInProgressMark),
                    "52" => Some(Catspm::MarkWithUnknownPurpose),
                    "53" => Some(Catspm::WellheadMark),
                    "54" => Some(Catspm::ChannelSeparationMark),
                    "55" => Some(Catspm::MarineFarmMark),
                    "56" => Some(Catspm::ArtificialReefMark),
                    _ => None,
                }
            }).filter(|ea| ea.is_some()).map(|ea| ea.unwrap()).collect::<Vec<Catspm>>()
        }).unwrap_or(vec![])
    }
}
