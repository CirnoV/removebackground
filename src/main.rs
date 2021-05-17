use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "removebackground")]
struct Opt {
    #[structopt(short = "o", long)]
    overwrite: bool,

    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    opt.files.iter().for_each(|path| {
        let img = image::open(&path).unwrap();
        let rgb8 = img.to_rgb8();

        let width = rgb8.width();
        let height = rgb8.height();
        let mut imgbuf = image::ImageBuffer::new(width, height);
        for (pixel_result, pixel) in imgbuf.pixels_mut().zip(rgb8.pixels()) {
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];
            let a = r.max(g).max(b);

            *pixel_result = image::Rgba([r, g, b, a]);
        }
        match opt.overwrite {
            true => {
                imgbuf.save(&path).unwrap();
            }
            false => {
                let file_stem = &path.file_stem().unwrap().to_str().unwrap();
                let extension = &path.extension().unwrap().to_str().unwrap();
                let filename = format!("{}_fix.{}", file_stem, extension);

                imgbuf.save(&filename).unwrap();
            }
        }
    });
}
