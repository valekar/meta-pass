use anchor_lang::prelude::*;

//test
declare_id!("5TzpQbN8Mec9c3aiX2KbK5jq6dYUnseDjuzzdjhcFZi4");

const PREFIX: &str = "meta-pass";
const META_PASS_ACCOUNT: &str = "meta_pass";
const EVENT_ACCOUNT: &str = "event";
const ORGANIZER_ACCOUNT: &str = "organizer";

#[program]
pub mod meta_pass {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, args: InitializeArgs) -> Result<()> {
        let meta_pass = &mut ctx.accounts.meta_pass;
        meta_pass.bump = *ctx.bumps.get(META_PASS_ACCOUNT).unwrap();
        meta_pass.authority = ctx.accounts.authority.key();
        meta_pass.convenience_fee = args.convenience_fee;
        meta_pass.treasury = ctx.accounts.treasury.key();

        Ok(())
    }

    pub fn initialize_organizer(
        ctx: Context<InitializeOrganizer>,
        args: InitializeOrganizerArgs,
    ) -> Result<()> {
        let organizer = &mut ctx.accounts.organizer;
        organizer.seq = args.seq;
        organizer.bump = *ctx.bumps.get(ORGANIZER_ACCOUNT).unwrap();
        organizer.authority = ctx.accounts.event_authority.key();
        Ok(())
    }

    //validation 
    pub fn create_event(ctx: Context<CreateEvent>, args: CreateEventArgs) -> Result<()> {
        let event = &mut ctx.accounts.event;
        event.bump = *ctx.bumps.get(EVENT_ACCOUNT).unwrap();
        event.event_authority = ctx.accounts.event_authority.key();
        event.config.name = args.name;
        event.config.description = args.description;
        event.config.start_timestamp = args.start_timestamp;
        event.config.duration_hours = args.duration_hours;
        event.config.event_type = args.event_type;
        event.config.ticket_price = args.ticket_price;
        event.config.total_tickets = args.total_tickets;

        //validation
        let organizer = &mut ctx.accounts.organizer;
        organizer.seq = organizer.seq.checked_add(1).unwrap();

        match args.token_mint {
            Some(value) => event.config.token_mint = value,
            None => {}
        }

        Ok(())
    }
}

// initialize meta pass
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        space = 8 + MetaPass::LEN ,
        seeds = [b"meta-pass".as_ref()],
        bump,
        payer = authority
    )]
    pub meta_pass: Account<'info, MetaPass>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub treasury: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeOrganizer<'info> {
    #[account(
        init,
        space = 8 + Organizer::LEN,
        seeds = [
            PREFIX.as_ref(),
            event_authority.key().as_ref(),
        ],
        bump,
        payer = event_authority
    )]
    pub organizer: Account<'info, Organizer>,

    #[account(mut)]
    pub event_authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

//create event
#[derive(Accounts)]
pub struct CreateEvent<'info> {
    #[account(
        init,
        space = 8 + Event::LEN,
        seeds = [
            PREFIX.as_ref(),
            event_authority.key().as_ref(),
            organizer.seq.to_le_bytes().as_ref()
        ],
        bump,
        payer = event_authority
    )]
    pub event: Account<'info, Event>,

    #[account(
        mut,
        seeds = [
            PREFIX.as_ref(),
            event_authority.key().as_ref(),
        ],
        bump = organizer.bump,
    )]
    pub organizer: Account<'info, Organizer>,


    #[account(mut)]
    pub event_authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// MetaPass
#[account]
pub struct MetaPass {
    pub authority: Pubkey,   // 32
    pub bump: u8,            // 1
    pub convenience_fee: u8, //1 // in percentage
    pub treasury: Pubkey,    // 32
}

impl MetaPass {
    pub const LEN: usize = 32 + 1 + 1 + 32;
}

// Organizer
#[account]
pub struct Organizer {
    bump: u8,          // 1
    seq: i64,          //16
    authority: Pubkey, //32
}

impl Organizer {
    pub const LEN: usize = 1 + 16 + 32;
}

// Events
#[account]
pub struct Event {
    pub event_authority: Pubkey, //32
    pub bump: u8,                // 1
    pub config: Config,          // 558
}

impl Event {
    pub const LEN: usize = 1 + 32 + Config::LEN;
}

#[derive(Clone, AnchorDeserialize, AnchorSerialize, PartialEq)]
pub struct Config {
    pub name: String,          // 200,
    pub description: String,   // 300,
    pub start_timestamp: i64,  // 16
    pub total_tickets: u32,    //4
    pub duration_hours: u8,    // 1
    pub event_type: EventType, // 1
    pub ticket_price: u32,     //4
    pub token_mint: Pubkey,    //32
}

impl Config {
    pub const LEN: usize = 200 + 300 + 16 + 4 + 1 + 1 + 4 + 32;
}

#[derive(Clone, AnchorDeserialize, AnchorSerialize, PartialEq)]
pub enum EventType {
    Online,
    Offline,
}

//arguments
#[account]
pub struct InitializeArgs {
    convenience_fee: u8,
}

#[account]
pub struct InitializeOrganizerArgs {
    seq: i64,
}

#[account]
pub struct CreateEventArgs {
    name: String,
    description: String,
    start_timestamp: i64,
    total_tickets: u32,
    duration_hours: u8,
    event_type: EventType,
    ticket_price: u32,
    token_mint: Option<Pubkey>,
}
