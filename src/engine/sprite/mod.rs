#[derive(Debug)]
struct Sprite {
    context: gl,
    url: &'static str,
    img: ImageElement,
}

impl Sprite {
    fn new(context: &mut Context, url: &str) -> Self {


    let ref tex = self.context.create_texture();
    self.context.bind_texture(gl::TEXTURE_2D, tex.as_ref());

    let img = ImageElement::new();
    img.set_src(&self.url);



        Sprite { context, url, img }
    }

    fn update() {
    self.context
        .bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));
    self.context
        .draw_elements(gl::TRIANGLES, 6, gl::UNSIGNED_SHORT, 0);

    self.context.bind_texture(gl::TEXTURE_2D, tex.as_ref());
    self.context.tex_image2_d_1(
        gl::TEXTURE_2D,
        0,
        gl::RGBA as i32,
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        &self.img,
    );

    self.context.generate_mipmap(gl::TEXTURE_2D);

    self.context
        .tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    self.context
        .tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    self.context
        .tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
    self.context
        .tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

    }
}
