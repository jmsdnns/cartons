use std::convert::Into;

use std::pin::Pin;
use std::sync::Arc;

use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use radio::radio_server::{Radio, RadioServer};
mod radio {
    tonic::include_proto!("radio");
}

mod data;

#[derive(Debug)]
pub struct RadioService {
    bandinfos: Arc<Vec<radio::BandInfo>>,
    songs: Arc<Vec<radio::Song>>,
}

#[tonic::async_trait]
impl Radio for RadioService {
    // Receives request and sends band info to client
    async fn get_band(
        &self,
        band_query: Request<radio::BandQuery>,
    ) -> Result<Response<radio::BandInfo>, Status> {
        let query = band_query.get_ref().name.clone();
        println!("[request] GetBand = {:?}", query);

        for bandinfo in &self.bandinfos[..] {
            if bandinfo.name == query {
                return Ok(Response::new(bandinfo.clone()));
            }
        }

        Ok(Response::new(radio::BandInfo::default()))
    }

    type ListSongsStream = ReceiverStream<Result<radio::Song, Status>>;

    // Receives request and streams songs to client
    async fn list_songs(
        &self,
        band_query: Request<radio::BandQuery>,
    ) -> Result<Response<Self::ListSongsStream>, Status> {
        let query = band_query.get_ref().name.clone();
        println!("[request] ListSongs = {:?}", query);

        let (tx, rx) = mpsc::channel(4);
        let songs: Arc<Vec<radio::Song>> = self.songs.clone();

        tokio::spawn(async move {
            for song in &songs[..] {
                if song.band.as_ref() == query {
                    println!("  => send {:?}", song);
                    tx.send(Ok(song.clone())).await.unwrap();
                }
            }

            println!("[/request]");
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    type RadioStream = Pin<Box<dyn Stream<Item = Result<radio::Song, Status>> + Send + 'static>>;

    // Receives streams of song requests, streams songs in response
    async fn radio(
        &self,
        request: Request<tonic::Streaming<radio::RadioNext>>,
    ) -> Result<Response<Self::RadioStream>, Status> {
        println!("[request] New Radio Stream");

        let mut stream = request.into_inner();
        let songs: Arc<Vec<radio::Song>> = self.songs.clone();

        let output = async_stream::try_stream! {
            while let Some(radio_next) = stream.next().await {
                let radio_next = radio_next?;

                let song = songs[radio_next.index as usize].clone();
                println!("  => send {:?}", song);

                yield song
            }
        };

        Ok(Response::new(Box::pin(output) as Self::RadioStream))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();

    println!("RadioServer listening on: {}", addr);
    let band_data: data::BandData = data::load();

    let ds_bandinfos = band_data
        .bandinfos
        .iter()
        .map(|bi| radio::BandInfo {
            name: bi.name.clone(),
            bio: bi.bio.clone(),
            members: bi
                .members
                .iter()
                .map(|bm| radio::BandMember {
                    name: bm.name.clone(),
                    role: bm.role.clone(),
                })
                .collect(),
        })
        .collect();

    let ds_songs = band_data
        .songs
        .iter()
        .map(|s| radio::Song {
            name: s.name.clone(),
            band: s.band.clone(),
            album: s.album.clone(),
        })
        .collect();

    let route_guide = RadioService {
        bandinfos: Arc::new(ds_bandinfos),
        songs: Arc::new(ds_songs),
    };

    let svc = RadioServer::new(route_guide);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
