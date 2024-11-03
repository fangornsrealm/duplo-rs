# Duplo-rs - Detect Similar or Duplicate Images and Videos

This Rust library allows you to perform a visual query on a set of images or videos, returning the results in the order of similarity. This allows you to effectively detect duplicates with minor modifications (e.g. some colour correction or watermarks) or text overlays.

It is an implementation of [Fast Multiresolution Image Querying](http://grail.cs.washington.edu/projects/query/mrquery.pdf) by Jacobs et al. which uses truncated Haar wavelet transforms to create visual hashes of the images. The same method has previously been used in the [imgSeek](http://www.imgseek.net) software and the [retrievr](http://labs.systemone.at/retrievr) website.

This project started as s re-implementation of [rivo/duplo](https://github.com/rivo/duplo) for the similar image technology that works for millions of images if it is allowed to run for weeks.

[duplo-rs](http://github.com/zuiopqewrt/duplo-rs) adds the ability to use the image search on videos and video segments. The video algorithm takes screenshots every 10 seconds and searches similar images for each. A video sequence is considered similar if at least six screenshots in a row match.

The user can fine-tune the sensitivity of the search algorithm with a value between 0 and 100.

- 100 means that only resized images will be seen as similar.
- 0 means that quite heavy modifications are allowed. Also it means that the center piece of the image can change a lot. For example a model shoot series will detect similar images with different poses of the model. The same setup with a different model might also match.

Make tests which level of sensitivity is *correct* for your use case. Consider allowing more matches (value closer to 0) when searching for videos. The pre-condition that six images in a row have to match is going to erase most of the *accidental* matches.

## Examples

There are example programs showing how this library can be used.

### demo_similar_images

```sh
cargo build --release --example demo_similar_images
./target/release/examples/demo_similar_images [options] <path to search>
```

This application reads every image in the directory or directory tree and compares it to the already read images. It compares the recent image with the best match and the better image is kept in the in-memory database. The pair of images is presented in a directory `simiilar_images`, the better one as a hard-link (will stay) the worse one will be moved (will go). The user can review the matches with any app that can display previews and sort files by name. If the match is good, just delete the files. If it isn't, copy the files that are marked `_REMOVE_` somewhere else if you want to keep them.

The storage ist persistent. So you can run the app again, using the same or a different start path. This app is designed to do comparisons that other programs just refuse to do. Most competitors will fall flat on their face with 100 000 images. This one has successfully searched 6 Million images and found 1 Million best matches. A job of this size will take multiple weeks. So a restart capability is nice to have.

### demo_similar_videos

```sh
cargo build --release --example demo_similar_videos
./target/release/examples/demo_similar_videos [options] <path to search>
```

This application reads every video in the directory or directory tree and compares it to the already read videos. The recent video is stored in the database so it doesn't have to be parsed again. When running the program again, the already read files are checked and the deleted files are removed from the database. For each new video it compares the screenshots with previously read videos. If it finds matches it exports a HTML file with previews and metadata into the directory `similar_videos` with the name of the new video. As there is no clear criterum no files are deleted. The users have to review the matches and decide for themselves.

The storage is persistent. So you can run the app again, using the same or a different start path.

## Usage

Include this in your Cargo.toml.

```toml
duplo-rs = "1.0.0"
```

Then call the library like this to search for images:

```rust
use duplo_rs;

// Create an empty store.
let mut store = duplo_rs::store::Store::new(sensitivity);

// Add image "img" to the store.
let hash = duplo_rs::files::process_image(file);
store.add("myimage", hash)

// Query the store based on image "query".
let (matches, failedid, failedhash) =
    duplo_rs::files::find_similar_images(&store, &filepath, &hash);
// matches[0] is the best match.
```

Or like this to search for videos:

```rust
use duplo_rs;

// Create an empty store.
let homedir_opt = dirs::home_dir();
if homedir_opt.is_none() {
    log::error!("Could not determine the home directory!");
    return;
}
let dbpath = homedir_opt.unwrap().join("similar_videos.sqlite3");
let dbpathstr = duplo_rs::files::osstring_to_string(dbpath.as_os_str());
let sql_client_opt = duplo_rs::videostore::connect(&dbpathstr);
if sql_client_opt.is_ok() {
    let mut sql_client = sql_client_opt.unwrap();
    let mut store = duplo_rs::videostore::VideoStore::new(
        &mut sql_client,
        sensitivity,
        &directory,
        num_threads,
    );
}
// parse the screenshots for movie and add data in "video" to the store.
let video = duplo_rs::files::process_video(file, video_id as usize, num_videos);
store.add(&mut sql_client, &video.id, &video, video.runtime);

// Query the store based on movie "video".
let (matches, failedid, _failedhash) =
    duplo_rs::files::find_similar_videos(&store, &mut sql_client, &video.id, &video);
// matches[0] is the best match.
```

## Documentation

[http://github.com/zuiopqewrt/duplo-rs](http://github.com/zuiopqewrt/duplo-rs)

## Possible Applications

- Identify copyright violations
- Save disk space by detecting and removing duplicate images or videos
- Search for images or videos by similarity
