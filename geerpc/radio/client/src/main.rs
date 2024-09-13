use std::error::Error;
use std::time::Duration;

use radio::{BandQuery, RadioNext};
use tokio::time;
use tonic::transport::Channel;
use tonic::Request;

use radio::radio_client::RadioClient;
pub mod radio {
    tonic::include_proto!("radio");
}

async fn run_get_band(
    client: &mut RadioClient<Channel>,
    name: String,
) -> Result<(), Box<dyn Error>> {
    let bq = BandQuery { name };

    let band = client.get_band(Request::new(bq)).await?.into_inner();

    println!("Band = {:?}", band);

    Ok(())
}

async fn run_list_songs(
    client: &mut RadioClient<Channel>,
    name: String,
) -> Result<(), Box<dyn Error>> {
    let bq = BandQuery { name };

    let mut stream = client.list_songs(Request::new(bq)).await?.into_inner();

    while let Some(song) = stream.message().await? {
        println!("Song = {:?}", song);
    }

    Ok(())
}

async fn run_radio(client: &mut RadioClient<Channel>) -> Result<(), Box<dyn Error>> {
    let outbound = async_stream::stream! {
        let mut interval = time::interval(Duration::from_secs(3));
        let mut index = 0;

        loop {
            interval.tick().await;

            let radio_next = RadioNext { index };
            println!("Next = {:?}", radio_next);
            yield radio_next;
            index += 1;
            if index == 4 {
                break;
            }
        }
    };

    let response = client.radio(Request::new(outbound)).await?;
    let mut inbound = response.into_inner();

    while let Some(song) = inbound.message().await? {
        println!("Song = {:?}", song);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RadioClient::connect("http://[::1]:10000").await?;

    println!("*** GET INFO ABOUT CONVERGE ***");
    run_get_band(&mut client, "Converge".to_string()).await?;

    println!("*** GET LIST OF CONVERGE SONGS ***");
    run_list_songs(&mut client, "Converge".to_string()).await?;

    println!("*** TURN ON THE RADIO ***");
    run_radio(&mut client).await?;

    Ok(())
}
