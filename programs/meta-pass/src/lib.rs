use anchor_lang::prelude::*;

declare_id!("5TzpQbN8Mec9c3aiX2KbK5jq6dYUnseDjuzzdjhcFZi4");

#[program]
pub mod meta_pass {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, args : InitializeArgs ) -> Result<()> {
        let meta_pass = &mut ctx.accounts.meta_pass;
        meta_pass.bump = *ctx.bumps.get("meta_pass").unwrap();
        meta_pass.authority = ctx.accounts.authority.key();
        meta_pass.convenience_fee = args.convenience_fee;
        meta_pass.treasury = ctx.accounts.treasury.key(); 

        Ok(())
    }

    pub fn create_event(ctx: Context<CreateEvent>, args: CreateEventArgs) -> Result<()> {
        let event = &mut ctx.accounts.event;
        event.bump = *ctx.bumps.get("event").unwrap();
        event.event_organizer = ctx.accounts.event_organizer.key();
        event.config.name = args.name;
        event.config.description = args.description;
        event.config.start_timestamp = args.start_timestamp;
        event.config.duration_hours = args.duration_hours;
        event.config.event_type = args.event_type;
        event.config.ticket_price = args.ticket_price;
        event.config.total_tickets = args.total_tickets;
        event.config.token_mint = args.token_mint;

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

    pub treasury : Signer<'info>,

    pub system_program: Program<'info, System>,
}

//create event
#[derive(Accounts)]
#[instruction(args : CreateEventArgs)]
pub struct CreateEvent<'info> {
    #[account(
        init,
        space = 8 + Event::LEN,
        seeds = [
            b"meta-pass".as_ref(),
            event_organizer.key().as_ref(),
            &args.timestamp.to_ne_bytes()
        ],
        bump,
        payer = event_organizer
    )]
    pub event: Account<'info, Event>,

    #[account(mut)]
    pub event_organizer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// MetaPass
#[account]
pub struct MetaPass {
    pub authority: Pubkey, // 32
    pub bump: u8,          // 1
    pub convenience_fee : u8, //1 // in percentage
    pub treasury : Pubkey // 32
}

impl MetaPass {
    pub const LEN: usize = 32 + 1 + 1;
}


// Events
#[account]
pub struct Event {
    pub event_organizer: Pubkey, //32
    pub bump: u8,                // 1
    pub config: Config,          // 558
}

impl Event {
    pub const LEN: usize = 1 + 32 + Config::LEN;
}

#[derive(Clone, AnchorDeserialize, AnchorSerialize, PartialEq)]
pub struct Config {
    name: String,          // 200,
    description: String,   // 300,
    start_timestamp: i64,  // 16
    total_tickets: u32,    //4
    duration_hours: u8,    // 1
    event_type: EventType, // 1
    ticket_price: u32,     //4
    token_mint: Pubkey,    //32
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
    convenience_fee : u8,
}


#[account]
pub struct CreateEventArgs {
    timestamp: i64,
    name: String,
    description: String,
    start_timestamp: i64,
    total_tickets: u32,
    duration_hours: u8,
    event_type: EventType,
    ticket_price: u32,
    token_mint: Pubkey,
}
