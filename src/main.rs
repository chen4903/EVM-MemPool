#![allow(non_snake_case)]
#![allow(unused_variables)]
mod execute;
mod listener;
mod utils;
mod ai;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "SecHelper", about = "A tool for assisting in monitoring, analyzing, and alerting blockchain security threats.")]
enum Cli {
    /// You AI security helper
    AI {
        /**********  OPTIONS    ***********/
        /// OpenAI API KEY
        #[structopt(short = "k", long = "key")] // OPTIONS
        openai_key: String,
        
        /// A domestic(For china) proxy springboard for accessing OpenAI
        #[structopt(short = "b", long = "baseurl")] // OPTIONS
        openai_base_url: String,
    },

    /// Robot to monitor
    Guardian {
        /**********  OPTIONS    ***********/
        /// Etherscan API kEY
        #[structopt(short = "k", long = "key")] // OPTIONS
        key: String,
        
        /// WSS URL
        #[structopt(short = "w", long = "wss")] // OPTIONS
        wss: String,

        /// Email from
        #[structopt(long = "sender")] // OPTIONS
        sender: String,

        /// Sender's email server password
        #[structopt(short = "p", long = "password")] // OPTIONS
        password: String,

        /// Email server smtp code
        #[structopt(short = "ss", long = "smtp_server")] // OPTIONS
        smtp_server: String,

        /// Who to monitor
        #[structopt()] // ARGS
        address: String,        
        
        /// Which email address to receive
        #[structopt()] // ARGS
        receiver: String,     

        /// The function you call. For warning_robot()
        #[structopt(default_value = "None")] // ARGS
        call: String,        
        
        /// The max number of certain txs. For warning_robot()
        #[structopt(default_value = "0")] // ARGS
        limit: u32,               

        /**********  FLAGS    ***********/
        /// message_robot
        #[structopt(long = "message_robot")] // FLAGS
        message_robot: bool,

        /// warning_robot
        #[structopt(long = "warning_robot")] // FLAGS
        warning_robot: bool,        
    },

    /// Fetch Blockchain data
    Fetcher {
        /**********  OPTIONS    ***********/
        /// Etherscan API kEY
        #[structopt(short = "k", long = "key")] // OPTIONS
        key: String,

        /// The address's txs you fetch
        #[structopt()] // ARGS
        address: String,     

        /// The blocko fetch txs from
        #[structopt(short = "s", long = "start")] // OPTIONS
        start_block: u64,

        /// The blocko fetch txs to
        #[structopt(short = "e", long = "end")] // OPTIONS
        end_block: u64,

        /// Obtain all transactions for a certain address
        #[structopt(short = "a", long = "all")] // FLAGS
        all: bool,

        /// Obtain normal transactions for a certain address
        #[structopt(short = "n", long = "normal")] // FLAGS
        normal: bool,

        /// Obtain internal transactions for a certain address
        #[structopt(short = "i", long = "internal")] // FLAGS
        internal: bool,

        /// Check that if an address is invoke to mixing service
        #[structopt(long = "mix")] // FLAGS
        is_invoke_mixing_service: bool,
    },

    /// Listen Blockchain data
    Listener {
        /**********  OPTIONS    ***********/
        /// Etherscan API kEY
        #[structopt(short = "k", long = "key")] // OPTIONS
        key: String,
        
        /// WSS URL
        #[structopt(short = "w", long = "wss")] // OPTIONS
        wss: String,

        /// The address to monitor
        #[structopt()] // ARGS
        address: String,     

        /// The event signature. For `subscribe_event()`
        #[structopt(default_value = "None")] // ARGS
        event: String,    

        /// Monitor a certain address if it has ERC20 transfer tx
        #[structopt(long = "sub_event")] // FLAGS
        subscribe_event: bool,

        /// Subscribe a certain address's all new txs
        #[structopt(long = "subaddress")] // FLAGS
        subscribe_address: bool,

        /// Monitor mixing service, record the users who interact with it
        #[structopt(short = "m", long = "monitor_mixing_service")] // FLAGS
        monitor_mixing_service: bool,
    },    
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::from_args();

    match cli {
        Cli::AI { openai_key, openai_base_url} => {
            let ai = ai::chatgpt::AI::new(openai_key, openai_base_url);
            ai.chatgpt().await;
        },
        Cli::Guardian { key, wss, sender, password, smtp_server, address, receiver, call, limit, message_robot, warning_robot} => {
            let guardian = execute::guardian::MessageRobot::new(key, wss, sender, password, smtp_server);

            if warning_robot == true { // warning_robot
                guardian.warning_robot(address.as_str(), call.as_str(), receiver, limit).await?;
            } else if message_robot == true{ // message_robot
                guardian.message_robot(address, receiver).await?;
            } else {

            }
        },
        Cli::Fetcher { key, address, start_block, end_block, all, normal, internal, is_invoke_mixing_service} => {
            let fetcher = listener::fetcher::Fetch::new(key);

            if  all == true {
                fetcher.fetch_address_all_txs(address.as_str(), start_block, end_block).await?;
            } else if normal == true {
                fetcher.fetch_address_normal_txs(address.as_str(), start_block, end_block).await?;
            } else if internal == true {
                fetcher.fetch_address_internal_txs(address.as_str(), start_block, end_block).await?;
            } else if is_invoke_mixing_service == true {
                let mix = fetcher.is_invoke_mixing_service(address.as_str(), start_block, end_block).await?;
                if mix == true {
                    println!("The {} is invoke mixing service!", address)
                }else {
                    println!("The {} is not invoke mixing service:)", address)
                }
            } else {
                println!("Not valid")
            }

        },
        Cli::Listener { key, wss, address, event,  subscribe_event, subscribe_address, monitor_mixing_service} => {
            let listener = listener::listen::Listen::new(wss, key);

            if subscribe_address == true {
                listener.subscribe_address(address).await?;
            } else if subscribe_event == true {
                listener.subscribe_event(address, event.as_str()).await?;
            } else if monitor_mixing_service == true {
                listener.monitor_mixing_service().await?;
            } else {
                println!("Invalid")
            }
        },
    }

    Ok(())
}
