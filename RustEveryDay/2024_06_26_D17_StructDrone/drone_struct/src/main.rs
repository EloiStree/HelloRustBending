use tungstenite::connect;
use tungstenite::Message;

fn main() {
    println!("Hello, world!");
    let websocket_uri = "ws://81.240.94.97:4504";

    let mut drones_soccer_12: [S_DronePositionCompressed; 12] = [
        S_DronePositionCompressed {
            local_position_x_from_center: 0,
            local_position_y_from_ground: 0,
            local_position_z_from_center: 0,
            euler_angle_x: 0,
            euler_angle_y: 0,
            euler_angle_z: 0,
        },
        S_DronePositionCompressed {
            local_position_x_from_center: 0,
            local_position_y_from_ground: 0,
            local_position_z_from_center: 0,
            euler_angle_x: 0,
            euler_angle_y: 0,
            euler_angle_z: 0,
        },
        S_DronePositionCompressed {
            local_position_x_from_center: 0,
            local_position_y_from_ground: 0,
            local_position_z_from_center: 0,
            euler_angle_x: 0,
            euler_angle_y: 0,
            euler_angle_z: 0,
        },
        S_DronePositionCompressed {
            local_position_x_from_center: 0,
            local_position_y_from_ground: 0,
            local_position_z_from_center: 0,
            euler_angle_x: 0,
            euler_angle_y: 0,
            euler_angle_z: 0,
        },
        S_DronePositionCompressed {
            local_position_x_from_center: 0,
            local_position_y_from_ground: 0,
            local_position_z_from_center: 0,
            euler_angle_x: 0,
            euler_angle_y: 0,
            euler_angle_z: 0,
        },
        S_DronePositionCompressed {
            local_position_x_from_center: 0,
            local_position_y_from_ground: 0,
            local_position_z_from_center: 0,
            euler_angle_x: 0,
            euler_angle_y: 0,
            euler_angle_z: 0,
        },
        S_DronePositionCompressed {
            local_position_x_from_center: 0,
            local_position_y_from_ground: 0,
            local_position_z_from_center: 0,
            euler_angle_x: 0,
            euler_angle_y: 0,
            euler_angle_z: 0,
        },
        S_DronePositionCompressed {
            local_position_x_from_center: 0,
            local_position_y_from_ground: 0,
            local_position_z_from_center: 0,
            euler_angle_x: 0,
            euler_angle_y: 0,
            euler_angle_z: 0,
        },
        S_DronePositionCompressed {
            local_position_x_from_center: 0,
            local_position_y_from_ground: 0,
            local_position_z_from_center: 0,
            euler_angle_x: 0,
            euler_angle_y: 0,
            euler_angle_z: 0,
        },
        S_DronePositionCompressed {
            local_position_x_from_center: 0,
            local_position_y_from_ground: 0,
            local_position_z_from_center: 0,
            euler_angle_x: 0,
            euler_angle_y: 0,
            euler_angle_z: 0,
        },
        S_DronePositionCompressed {
            local_position_x_from_center: 0,
            local_position_y_from_ground: 0,
            local_position_z_from_center: 0,
            euler_angle_x: 0,
            euler_angle_y: 0,
            euler_angle_z: 0,
        },
        S_DronePositionCompressed {
            local_position_x_from_center: 0,
            local_position_y_from_ground: 0,
            local_position_z_from_center: 0,
            euler_angle_x: 0,
            euler_angle_y: 0,
            euler_angle_z: 0,
        },


    ];


    let mut drone_positions:S_DroneSoccerPositions= S_DroneSoccerPositions{
        date_time_utc_tick: 0,
        frame_pushed: 0,
        red_drone0_stricker: drones_soccer_12[0],
        red_drone1: drones_soccer_12[1],
        red_drone2: drones_soccer_12[2],
        red_drone3: drones_soccer_12[3],
        red_drone4: drones_soccer_12[4],
        red_drone5: drones_soccer_12[5],
        blue_drone0_stricker: drones_soccer_12[6],
        blue_drone1: drones_soccer_12[7],
        blue_drone2: drones_soccer_12[8],
        blue_drone3: drones_soccer_12[9],
        blue_drone4: drones_soccer_12[10],
        blue_drone5: drones_soccer_12[11],
    };





    let mut last_struct_received: S_All= S_All{
        s_drone_positions: drone_positions,
        s_public_elliptic_curve_claim   : S_DroneSoccerPublicXmlRsaKey1024Claim{
            red_drone0_stricker: "".to_string(),
            red_drone1: "".to_string(),
            red_drone2: "".to_string(),
            red_drone3: "".to_string(),
            red_drone4: "".to_string(),
            red_drone5: "".to_string(),
            blue_drone0_stricker: "".to_string(),
            blue_drone1: "".to_string(),
            blue_drone2: "".to_string(),
            blue_drone3: "".to_string(),
            blue_drone4: "".to_string(),
            blue_drone5: "".to_string(),
        },
        s_match_time_value: S_DroneSoccerTimeValue{
            seconds_since_match_started: 0.0,
            seconds_since_set_started: 0.0,
            time_of_server_date_time_utc_now_ticks: 0,
        },
        s_soccer_ball_position: S_DroneSoccerBallPosition{
            date_time_utc_tick: 0,
            position: Vector3{x: 0.0, y: 0.0, z: 0.0},
            rotation: Quaternion{x: 0.0, y: 0.0, z: 0.0, w: 0.0},
        },
        s_points_state: S_DroneSoccerMatchState{
            blue_points: 0,
            red_points: 0,
            blue_sets: 0,
            red_sets: 0,
            utc_tick_in_seconds_when_match_started: 0,
            utc_tick_in_seconds_when_match_finished: 0,
        },
        s_index_integer_claim: S_DroneSoccerIndexIntegerClaim{
            red_drone0_stricker: 0,
            red_drone1: 0,
            red_drone2: 0,
            red_drone3: 0,
            red_drone4: 0,
            red_drone5: 0,
            blue_drone0_stricker: 0,
            blue_drone1: 0,
            blue_drone2: 0,
            blue_drone3: 0,
            blue_drone4: 0,
            blue_drone5: 0,
        },
        s_public_rsa_key_claim: S_DroneSoccerPublicXmlRsaKey1024Claim{
            red_drone0_stricker: "".to_string(),
            red_drone1: "".to_string(),
            red_drone2: "".to_string(),
            red_drone3: "".to_string(),
            red_drone4: "".to_string(),
            red_drone5: "".to_string(),
            blue_drone0_stricker: "".to_string(),
            blue_drone1: "".to_string(),
            blue_drone2: "".to_string(),
            blue_drone3: "".to_string(),
            blue_drone4: "".to_string(),
            blue_drone5: "".to_string(),
        },
        s_arena_static_information: S_DroneSoccerMatchStaticInformation{
            max_timing_of_set_in_seconds: 0.0,
            max_timing_of_match_in_seconds: 0.0,
            number_of_sets_to_win_match: 0.0,
            number_of_points_to_force_win_set: 0.0,
            arena_width_meter: 0.0,
            arena_height_meter: 0.0,
            arena_depth_meter: 0.0,
            goal_distance_of_center_meter: 0.0,
            goal_center_height_meter: 0.0,
            goal_inner_radius_meter: 0.0,
            goal_outer_radius_meter: 0.0,
            goal_depth_meter: 0.0,
            drone_sphere_radius_meter: 0.0,
        },
        s_soccer_ball_goals: S_DroneSoccerBallGoals{
            goal_distance_of_center_meter: 0.0,
            goal_ground_height_meter: 0.0,
            goal_width_radius_meter: 0.0,
            goal_depth_meter: 0.0,
            ball_radius: 0.0,
        },
        s_projectile_creation: S_LinearProjectilePoolItemCreationEvent{
            pool_id: 0,
            pool_item_index: 0,
            server_utc_now_ticks: 0,
            start_position: Vector3{x: 0.0, y: 0.0, z: 0.0},
            start_rotation: Quaternion{x: 0.0, y: 0.0, z: 0.0, w: 0.0},
            start_direction: Vector3{x: 0.0, y: 0.0, z: 0.0},
            speed_in_meters_per_second: 0.0,
            collider_radius: 0.0,
        },
        s_projectile_destruction: S_PoolItemDestructionEvent{
            pool_id: 0,
            pool_item_index: 0,
            server_utc_now_ticks: 0,
        },
        s_server_frame_time: S_NetworkGameFramePushTiming{
            utc_now_tick_server: 0,
            game_network_frame: 0,
        },
        s_double_guid_item_spawn: S_DoubleGuidItemSpawn{
            server_utc_time_ticks_now: 0,
            item_string: "".to_string(),
            prefab_string: "".to_string(),
            item_guid_as_bytes: vec![],
            prefab_guid_as_bytes: vec![],
            arena_position: Vector3{x: 0.0, y: 0.0, z: 0.0},
            arena_rotation: Quaternion{x: 0.0, y: 0.0, z: 0.0, w: 0.0},
            arena_scale: Vector3{x: 0.0, y: 0.0, z: 0.0},
        },
        s_double_guid_item_destruction: S_DoubleGuidItemDestruction{
            server_utc_time_ticks_now: 0,
            item_string: "".to_string(),
            prefab_string: "".to_string(),
            item_guid_as_bytes: vec![],
            prefab_guid_as_bytes: vec![],
        }
    };

    let first_byte_category_type: CPS_BytePerType  = CPS_BytePerType {
        byte_drone_positions: 10,
        byte_match_time_value: 11,
        byte_soccer_ball_position: 15,
        byte_points_state: 20,
        byte_index_integer_claim: 60,
        byte_public_rsa_key_claim: 61,
        byte_public_elliptic_curve_claim: 62,
        byte_arena_static_information: 3,
        byte_soccer_ball_goals: 35,
        byte_projectile_creation: 70,
        byte_projectile_destruction: 71,
        byte_server_frame_time: 9,
        byte_double_guid_item_spawn: 80,
        byte_double_guid_item_destruction: 81,
    };

    let (mut socket, _) = connect(websocket_uri).expect("Failed to connect to websocket");

    loop {
        let msg = socket.read().expect("Failed to read message from websocket");
        

        if let Message::Binary(data) = msg {
            if(data.len() > 0){
               let byte_category = data[0];
               

            match byte_category {
                10 => {
                    //let drone_positions: S_DroneSoccerPositions = bincode::deserialize(&data[1..]).expect("Failed to deserialize drone positions");
                    // Process drone positions

                },
                11 => {
                    //let match_time_value: S_DroneSoccerTimeValue = bincode::deserialize(&data[1..]).expect("Failed to deserialize match time value");
                    // Process match time value
                },
                15 => {
                    //let soccer_ball_position: S_DroneSoccerBallPosition = bincode::deserialize(&data[1..]).expect("Failed to deserialize soccer ball position");
                    // Process soccer ball position
                },
                20 => {
                    //let points_state: S_DroneSoccerMatchState = bincode::deserialize(&data[1..]).expect("Failed to deserialize points state");
                    // Process points state
                },
                60 => {
                    //let index_integer_claim: S_DroneSoccerIndexIntegerClaim = bincode::deserialize(&data[1..]).expect("Failed to deserialize index integer claim");
                    // Process index integer claim
                },
                61 => {
                    //let public_rsa_key_claim: S_DroneSoccerPublicXmlRsaKey1024Claim = bincode::deserialize(&data[1..]).expect("Failed to deserialize public RSA key claim");
                    // Process public RSA key claim
                },
                62 => {
                    //let public_elliptic_curve_claim: S_DroneSoccerPublicXmlRsaKey1024Claim = bincode::deserialize(&data[1..]).expect("Failed to deserialize public elliptic curve claim");
                    // Process public elliptic curve claim
                },
                3 => {
                    //let arena_static_information: S_DroneSoccerMatchStaticInformation = bincode::deserialize(&data[1..]).expect("Failed to deserialize arena static information");
                    // Process arena static information
                },
                35 => {
                    //let soccer_ball_goals: S_DroneSoccerBallGoals = bincode::deserialize(&data[1..]).expect("Failed to deserialize soccer ball goals");
                    // Process soccer ball goals
                },
                70 => {
                    //let projectile_creation: S_LinearProjectilePoolItemCreationEvent = bincode::deserialize(&data[1..]).expect("Failed to deserialize projectile creation");
                    // Process projectile creation
                },
                71 => {
                    //let projectile_destruction: S_PoolItemDestructionEvent = bincode::deserialize(&data[1..]).expect("Failed to deserialize projectile destruction");
                    // Process projectile destruction
                },
                9 => {
                    //let server_frame_time: S_NetworkGameFramePushTiming = bincode::deserialize(&data[1..]).expect("Failed to deserialize server frame time");
                    // Process server frame time
                },
                80 => {
                    //let double_guid_item_spawn: S_DoubleGuidItemSpawn = bincode::deserialize(&data[1..]).expect("Failed to deserialize double guid item spawn");
                    // Process double guid item spawn
                },
                81 => {
                    //let double_guid_item_destruction: S_DoubleGuidItemDestruction = bincode::deserialize(&data[1..]).expect("Failed to deserialize double guid item destruction");
                    // Process double guid item destruction
                },
                _ => {
                    // Unknown byte category, handle accordingly
                }
            }



             }
             else {



             }
        }
    }
    //println!("Bye, world!");
}




pub struct CPS_BytePerType {
    pub byte_drone_positions: u8,
    pub byte_match_time_value: u8,
    pub byte_soccer_ball_position: u8,
    pub byte_points_state: u8,
    pub byte_index_integer_claim: u8,
    pub byte_public_rsa_key_claim: u8,
    pub byte_public_elliptic_curve_claim: u8,
    pub byte_arena_static_information: u8,
    pub byte_soccer_ball_goals: u8,
    pub byte_projectile_creation: u8,
    pub byte_projectile_destruction: u8,
    pub byte_server_frame_time: u8,
    pub byte_double_guid_item_spawn: u8,
    pub byte_double_guid_item_destruction: u8,
}

#[derive(Debug, Clone)]
pub  struct  S_All{
    pub s_drone_positions: S_DroneSoccerPositions,
    pub s_match_time_value: S_DroneSoccerTimeValue,
    pub s_soccer_ball_position: S_DroneSoccerBallPosition,
    pub s_points_state: S_DroneSoccerMatchState,
    pub s_index_integer_claim: S_DroneSoccerIndexIntegerClaim,
    pub s_public_rsa_key_claim: S_DroneSoccerPublicXmlRsaKey1024Claim,
    pub s_public_elliptic_curve_claim: S_DroneSoccerPublicXmlRsaKey1024Claim,
    pub s_arena_static_information: S_DroneSoccerMatchStaticInformation,
    pub s_soccer_ball_goals: S_DroneSoccerBallGoals,
    pub s_projectile_creation: S_LinearProjectilePoolItemCreationEvent,
    pub s_projectile_destruction:   S_PoolItemDestructionEvent,
    pub s_server_frame_time: S_NetworkGameFramePushTiming,
    pub s_double_guid_item_spawn: S_DoubleGuidItemSpawn,
    pub s_double_guid_item_destruction: S_DoubleGuidItemDestruction,
}


#[derive(Debug, Clone, Copy)]
pub struct S_PoolItemDestructionEvent {
    pub pool_id: u8,
    pub pool_item_index: u32,
    pub server_utc_now_ticks: u64,
}


#[derive(Debug, Clone, Copy)]
pub struct S_NetworkGameFramePushTiming {
    pub utc_now_tick_server: u64,
    pub game_network_frame: u64,
}
#[derive(Debug, Clone, Copy)]
pub struct S_DronePositionCompressed {
    pub local_position_x_from_center: i16,
    pub local_position_y_from_ground: u16,
    pub local_position_z_from_center: i16,
    pub euler_angle_x: u8,
    pub euler_angle_y: u8,
    pub euler_angle_z: u8,
}
#[derive(Debug, Clone)]
pub struct S_DoubleGuidItemDestruction {
    pub server_utc_time_ticks_now: u64,
    pub item_string: String,
    pub prefab_string: String,
    pub item_guid_as_bytes: Vec<u8>,
    pub prefab_guid_as_bytes: Vec<u8>,
}
#[derive(Debug, Clone)]
pub struct S_DoubleGuidItemSpawn {
    pub server_utc_time_ticks_now: u64,
    pub item_string: String,
    pub prefab_string: String,
    pub item_guid_as_bytes: Vec<u8>,
    pub prefab_guid_as_bytes: Vec<u8>,
    pub arena_position: Vector3,
    pub arena_rotation: Quaternion,
    pub arena_scale: Vector3,
}
#[derive(Debug, Clone, Copy)]
pub struct S_DroneSoccerTimeValue {
    pub seconds_since_match_started: f32,
    pub seconds_since_set_started: f32,
    pub time_of_server_date_time_utc_now_ticks: u64,
}
#[derive(Debug, Clone, Copy)]
pub struct S_DroneSoccerBallGoals {
    pub goal_distance_of_center_meter: f32,
    pub goal_ground_height_meter: f32,
    pub goal_width_radius_meter: f32,
    pub goal_depth_meter: f32,
    pub ball_radius: f32,
}
#[derive(Debug, Clone, Copy)]
pub struct S_DroneSoccerBallPosition {
    pub date_time_utc_tick: u64,
    pub position: Vector3,
    pub rotation: Quaternion,
}
#[derive(Debug, Clone, Copy)]
pub struct S_DroneSoccerIndexIntegerClaim {
    pub red_drone0_stricker: i32,
    pub red_drone1: i32,
    pub red_drone2: i32,
    pub red_drone3: i32,
    pub red_drone4: i32,
    pub red_drone5: i32,
    pub blue_drone0_stricker: i32,
    pub blue_drone1: i32,
    pub blue_drone2: i32,
    pub blue_drone3: i32,
    pub blue_drone4: i32,
    pub blue_drone5: i32,
}
#[derive(Debug, Clone, Copy)]
pub struct S_DroneSoccerMatchState {
    pub blue_points: u32,
    pub red_points: u32,
    pub blue_sets: u32,
    pub red_sets: u32,
    pub utc_tick_in_seconds_when_match_started: u64,
    pub utc_tick_in_seconds_when_match_finished: u64,
}
#[derive(Debug, Clone, Copy)]
pub struct S_DroneSoccerMatchStaticInformation {
    pub max_timing_of_set_in_seconds: f32,
    pub max_timing_of_match_in_seconds: f32,
    pub number_of_sets_to_win_match: f32,
    pub number_of_points_to_force_win_set: f32,
    pub arena_width_meter: f32,
    pub arena_height_meter: f32,
    pub arena_depth_meter: f32,
    pub goal_distance_of_center_meter: f32,
    pub goal_center_height_meter: f32,
    pub goal_inner_radius_meter: f32,
    pub goal_outer_radius_meter: f32,
    pub goal_depth_meter: f32,
    pub drone_sphere_radius_meter: f32,
}
#[derive(Debug, Clone, Copy)]
pub struct S_DroneSoccerPositions {
    pub date_time_utc_tick: u64,
    pub frame_pushed: u64,
    pub red_drone0_stricker: S_DronePositionCompressed,
    pub red_drone1: S_DronePositionCompressed,
    pub red_drone2: S_DronePositionCompressed,
    pub red_drone3: S_DronePositionCompressed,
    pub red_drone4: S_DronePositionCompressed,
    pub red_drone5: S_DronePositionCompressed,
    pub blue_drone0_stricker: S_DronePositionCompressed,
    pub blue_drone1: S_DronePositionCompressed,
    pub blue_drone2: S_DronePositionCompressed,
    pub blue_drone3: S_DronePositionCompressed,
    pub blue_drone4: S_DronePositionCompressed,
    pub blue_drone5: S_DronePositionCompressed,
}
#[derive(Debug, Clone)]
pub struct S_DroneSoccerPublicXmlRsaKey1024Claim {
    pub red_drone0_stricker: String,
    pub red_drone1: String,
    pub red_drone2: String,
    pub red_drone3: String,
    pub red_drone4: String,
    pub red_drone5: String,
    pub blue_drone0_stricker: String,
    pub blue_drone1: String,
    pub blue_drone2: String,
    pub blue_drone3: String,
    pub blue_drone4: String,
    pub blue_drone5: String,
}
#[derive(Debug, Clone, Copy)]
pub struct S_LinearProjectilePoolItemCreationEvent {
    pub pool_id: u8,
    pub pool_item_index: u32,
    pub server_utc_now_ticks: u64,
    pub start_position: Vector3,
    pub start_rotation: Quaternion,
    pub start_direction: Vector3,
    pub speed_in_meters_per_second: f32,
    pub collider_radius: f32,
}
#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

/*



[System.Serializable]
public struct S_PoolItemDestructionEvent
{
    public byte m_poolId;
    public uint m_poolItemIndex;
    public ulong m_serverUtcNowTicks;
}
//12 time per second refresh
[System.Serializable]
public struct S_NetworkGameFramePushTiming {

    public ulong m_utcNowTickServer;
    public ulong m_gameNetworkFrame;
}



[System.Serializable]
public struct S_DronePositionCompressed
{
    public short m_localPositionXFromCenter;
    public ushort m_localPositionYFromGround;
    public short m_localPositionZFromCenter;
    public byte m_eulerAngleX;
    public byte m_eulerAngleY;
    public byte m_eulerAngleZ;
}


[System.Serializable]
public struct S_DoubleGuidItemDestruction 
{
    public ulong m_serverUtcTimeTicksNow;
    public string m_itemString;
    public string m_prefabString;
    public byte[] m_itemGuidAsBytes;
    public byte[] m_prefabGuidAsBytes;
    
}
[System.Serializable]
public struct S_DoubleGuidItemSpawn 
{
    public ulong m_serverUtcTimeTicksNow;
    public string m_itemString;
    public string m_prefabString;
    public byte[] m_itemGuidAsBytes;
    public byte[] m_prefabGuidAsBytes;
    public Vector3 m_arenaPosition;
    public Quaternion m_arenaRotation;
    public Vector3 m_arenaScale;
}

[System.Serializable]
public struct S_DroneSoccerTimeValue {
    public float m_secondsSinceMatchStarted;
    public float m_secondsSinceSetStarted;
    public ulong m_timeOfServerDateTimeUtcNowTicks;

   
}

[System.Serializable]
public struct S_DroneSoccerBallGoals {

    public float m_goalDistanceOfCenterMeter;
    public float m_goalGroundHeightMeter;
    public float m_goalWidthRadiusMeter;
    public float m_goalDepthMeter;
    public float m_ballRadius;

  
    
    
}

using UnityEngine;

[System.Serializable]
public struct S_DroneSoccerBallPosition
{
    public ulong m_dateTimeUtcTick;
    public Vector3 m_position;
    public Quaternion m_rotation;


}


    [System.Serializable]
public struct S_DroneSoccerIndexIntegerClaim
{
    public int m_redDrone0Stricker;
    public int m_redDrone1;
    public int m_redDrone2;
    public int m_redDrone3;
    public int m_redDrone4;
    public int m_redDrone5;
    public int m_blueDrone0Stricker;
    public int m_blueDrone1;
    public int m_blueDrone2;
    public int m_blueDrone3;
    public int m_blueDrone4;
    public int m_blueDrone5;

  
}


    //Refresh with event
[System.Serializable]
public struct S_DroneSoccerMatchState
{   
    public uint m_bluePoints;
    public uint m_redPoints;
    public uint m_blueSets;
    public uint m_redSets;
    public ulong m_utcTickInSecondsWhenMatchStarted;
    public ulong m_utcTickInSecondsWhenMatchFinished;

   
    
    
}


    using UnityEngine;
//Set Once at start of the match
[System.Serializable]
public struct S_DroneSoccerMatchStaticInformation {
    [Header("Match Info")]
    public float m_maxTimingOfSetInSeconds;//300 seconds
    public float m_maxTimingOfMatchInSeconds;//15 minutes
    public float m_numberOfSetsToWinMatch;//2 sets
    [Tooltip("If a team reach this points, the set is won")]
    public float m_numberOfPointsToForceWinSet;//99 points

    [Header("Dimension")]
    public float m_arenaWidthMeter;//around 7 meters
    public float m_arenaHeightMeter;//around 6 meters
    public float m_arenaDepthMeter;//around 14 meters
    public float m_goalDistanceOfCenterMeter;//4 meters+
    public float m_goalCenterHeightMeter;//3 meters
    public float m_goalInnerRadiusMeter;// 60cm+
    public float m_goalOuterRadiusMeter;///70cm+
    public float m_goalDepthMeter;//around 5-20 cm
    public float m_droneSphereRadiusMeter;//40cm 0.4 meter


  
}

//12 time per second refresh

[System.Serializable]
public struct S_DroneSoccerPositions
{
    public ulong m_dateTimeUtcTick;
    public ulong m_framePushed;
    public S_DronePositionCompressed m_redDrone0Stricker;
    public S_DronePositionCompressed m_redDrone1;
    public S_DronePositionCompressed m_redDrone2;
    public S_DronePositionCompressed m_redDrone3;
    public S_DronePositionCompressed m_redDrone4;
    public S_DronePositionCompressed m_redDrone5;
    public S_DronePositionCompressed m_blueDrone0Stricker;
    public S_DronePositionCompressed m_blueDrone1;
    public S_DronePositionCompressed m_blueDrone2;
    public S_DronePositionCompressed m_blueDrone3;
    public S_DronePositionCompressed m_blueDrone4;
    public S_DronePositionCompressed m_blueDrone5;

   
}

[System.Serializable]
public struct S_DroneSoccerPublicXmlRsaKey1024Claim
{
    public string m_redDrone0Stricker;
    public string m_redDrone1;
    public string m_redDrone2;
    public string m_redDrone3;
    public string m_redDrone4;
    public string m_redDrone5;
    public string m_blueDrone0Stricker;
    public string m_blueDrone1;
    public string m_blueDrone2;
    public string m_blueDrone3;
    public string m_blueDrone4;
    public string m_blueDrone5;

}

//Refresh every seconds
using UnityEngine;

[System.Serializable]
public struct S_LinearProjectilePoolItemCreationEvent
{
    public byte m_poolId;
    public uint m_poolItemIndex;
    public ulong m_serverUtcNowTicks;
    public Vector3 m_startPosition;
    public Quaternion m_startRotation;
    public Vector3 m_startDirection;
    public float m_speedInMetersPerSecond;
    public float m_colliderRadius;
}


*/
