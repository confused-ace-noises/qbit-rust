use crate::{api_fns::torrents::torrents::TorrentType, auth::api::Api};

use super::{add_torrent::TorrentAddDescriptor, torrents::Torrent};

#[test]
fn prob_the_only_time_ive_ever_prayed() {
    let x = TorrentAddDescriptor::builder()
        .torrents(vec![Torrent::new("aaaaaa", TorrentType::Url).unwrap(), Torrent::new("/home/arch/hiiiii", TorrentType::RawTorrent).unwrap()])
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
        
    println!("{:?}", x)
}