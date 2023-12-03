use std::{future::IntoFuture, net::SocketAddr};

use super::CLA;
use anyhow::Context;
use routers::{router_games, router_pages};
use tokio::net::TcpListener;

mod routers;
mod routes;

pub struct Server {
    port_pages: u16,
    port_games: u16,
}

impl Server {
    pub async fn run(&mut self) -> anyhow::Result<()> {
        let pages = router_pages();
        let games = router_games();

        let addr_pages = SocketAddr::from(([0, 0, 0, 0], self.port_pages));
        let addr_games = SocketAddr::from(([0, 0, 0, 0], self.port_games));

        let listen_pages = TcpListener::bind(addr_pages).await.unwrap();

        let listen_games = TcpListener::bind(addr_games).await.unwrap();

        // let a = axum::serve(listen_pages, pages);

        let (res_pages, res_games) = tokio::join!(
            axum::serve(listen_pages, pages).into_future(),
            axum::serve(listen_games, games).into_future(),
        );

        let (res_pages, res_games) = (
            res_pages.context("Error while creating pages server"),
            res_games.context("Error while creating games server"),
        );

        let res = match (res_pages, res_games) {
            (Err(e), _) => Err(e),
            (Ok(()), Err(e)) => Err(e),
            (Ok(()), Ok(())) => Ok(()),
        };
        return res;
    }
}

impl From<&CLA> for Server {
    fn from(args: &CLA) -> Self {
        Self {
            port_pages: args.port_pages,
            port_games: args.port_games,
        }
    }
}
