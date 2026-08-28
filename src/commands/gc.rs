use std::net::UdpSocket;

pub const GAMECONTROLLER_DATA_PORT: u16 = 3838;
pub const GAMECONTROLLER_STRUCT_HEADER: &[u8; 4] = b"RGme";
pub const GAMECONTROLLER_STRUCT_VERSION: u8 = 20;

// Clean, standard Rust structs
#[derive(Debug)]
pub struct RobotInfo {
    pub penalty: u8,
    pub secs_till_unpenalised: u8,
    pub cautions: u8,
}

#[derive(Debug)]
pub struct TeamInfo {
    pub team_number: u8,
    pub field_player_colour: u8,
    pub goalkeeper_colour: u8,
    pub goalkeeper: u8,
    pub score: u8,
    pub penalty_shot: u8,
    pub single_shots: u16,
    pub message_budget: u16,
    pub players: Vec<RobotInfo>,
}

#[derive(Debug)]
pub struct RoboCupGameControlData {
    pub header: [u8; 4],
    pub version: u8,
    pub packet_number: u8,
    pub players_per_team: u8,
    pub competition_type: u8,
    pub stopped: bool,
    pub game_phase: u8,
    pub state: u8,
    pub set_play: u8,
    pub first_half: bool,
    pub kicking_team: u8,
    pub secs_remaining: i16,
    pub secondary_time: i16,
    pub teams: [TeamInfo; 2],
}

// Helper struct to advance through the byte slice cleanly
struct Reader<'a> {
    slice: &'a [u8],
}

impl<'a> Reader<'a> {
    fn new(slice: &'a [u8]) -> Self {
        Self { slice }
    }

    fn read_u8(&mut self) -> Option<u8> {
        let (val, rest) = self.slice.split_first()?;
        self.slice = rest;
        Some(*val)
    }

    fn read_u16(&mut self) -> Option<u16> {
        if self.slice.len() < 2 {
            return None;
        }
        let val = u16::from_le_bytes([self.slice[0], self.slice[1]]);
        self.slice = &self.slice[2..];
        Some(val)
    }

    fn read_i16(&mut self) -> Option<i16> {
        if self.slice.len() < 2 {
            return None;
        }
        let val = i16::from_le_bytes([self.slice[0], self.slice[1]]);
        self.slice = &self.slice[2..];
        Some(val)
    }

    fn read_team(&mut self) -> Option<TeamInfo> {
        let team_number = self.read_u8()?;
        let field_player_colour = self.read_u8()?;
        let goalkeeper_colour = self.read_u8()?;
        let goalkeeper = self.read_u8()?;
        let score = self.read_u8()?;
        let penalty_shot = self.read_u8()?;
        let single_shots = self.read_u16()?;
        let message_budget = self.read_u16()?;

        let mut players = Vec::with_capacity(20);
        for _ in 0..20 {
            players.push(RobotInfo {
                penalty: self.read_u8()?,
                secs_till_unpenalised: self.read_u8()?,
                cautions: self.read_u8()?,
            });
        }

        Some(TeamInfo {
            team_number,
            field_player_colour,
            goalkeeper_colour,
            goalkeeper,
            score,
            penalty_shot,
            single_shots,
            message_budget,
            players,
        })
    }
}

impl RoboCupGameControlData {
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        let mut reader = Reader::new(bytes);

        let header = [
            reader.read_u8()?,
            reader.read_u8()?,
            reader.read_u8()?,
            reader.read_u8()?,
        ];
        let version = reader.read_u8()?;
        let packet_number = reader.read_u8()?;
        let players_per_team = reader.read_u8()?;
        let competition_type = reader.read_u8()?;
        let stopped = reader.read_u8()? != 0;
        let game_phase = reader.read_u8()?;
        let state = reader.read_u8()?;
        let set_play = reader.read_u8()?;
        let first_half = reader.read_u8()? != 0;
        let kicking_team = reader.read_u8()?;
        let secs_remaining = reader.read_i16()?;
        let secondary_time = reader.read_i16()?;

        let team1 = reader.read_team()?;
        let team2 = reader.read_team()?;

        Some(RoboCupGameControlData {
            header,
            version,
            packet_number,
            players_per_team,
            competition_type,
            stopped,
            game_phase,
            state,
            set_play,
            first_half,
            kicking_team,
            secs_remaining,
            secondary_time,
            teams: [team1, team2],
        })
    }
}

/// Clones a repository and runs a list of setup commands inside its directory.
pub fn receive() -> std::io::Result<()> {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", GAMECONTROLLER_DATA_PORT))?;
    println!(
        "Listening on port {} (Safe Reader Parser)...",
        GAMECONTROLLER_DATA_PORT
    );

    let mut buf = [0u8; 2048];

    loop {
        let (bytes_len, src) = socket.recv_from(&mut buf)?;

        if let Some(gc_data) = RoboCupGameControlData::parse(&buf[..bytes_len])
            && &gc_data.header == GAMECONTROLLER_STRUCT_HEADER
            && gc_data.version == GAMECONTROLLER_STRUCT_VERSION
        {
            println!(
                "[{src}] Secs Left: {}s | Score: {} - {} | State: {} | {} vs {}",
                gc_data.secs_remaining,
                gc_data.teams[0].score,
                gc_data.teams[1].score,
                gc_data.state,
                gc_data.teams[0].team_number,
                gc_data.teams[1].team_number
            );
        }
    }
}
