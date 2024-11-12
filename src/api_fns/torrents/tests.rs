use crate::{api_fns::torrents::torrents::TorrentType, auth::{api::Api, creds::Credentials}};

use super::{add_torrent::TorrentAddDescriptor, torrents::Torrent};

#[test]
fn prob_the_only_time_ive_ever_prayed() {
    let x = TorrentAddDescriptor::builder()
        .torrents(vec![Torrent::new("aaaaaa", TorrentType::Url), Torrent::new("/home/arch/hiiiii", TorrentType::RawTorrent)])
        .savepath("/downloads".to_string())
        .cookie("my_cookie_value")
        .category("Movies")
        .tags(vec!["action", "comedy"])
        .skip_checking(true)
        .paused(false)
        .root_folder("root_folder")
        .rename("my_torrent")
        .up_limit(1024)
        .dl_limit(2048)
        .ratio_limit(2.5)
        .seeding_time_limit(120)
        .auto_tmm(true)
        .sequential_download(true)
        .first_last_piece_prio(false)
        .build().unwrap();
        
    println!("{:?}a", x)
}

#[tokio::test]
async fn nvm_this_is_the_second_time_ive_ever_prayed() {
    let api = Api::new("http://localhost:6011/", Credentials::new("admin", "123456")).await.unwrap();
    api.add_torrent(TorrentAddDescriptor::builder().torrents(vec![Torrent::new("/home/arch/Downloads/archlinux-2024.11.01-x86_64.iso.torrent", TorrentType::RawTorrent)]).tags(vec!["aaaa1", "aaaaaa2"]).rename("hiiiii").build().unwrap()).await.unwrap();
}