
use core::libc::{c_float, c_uint};
use system::vector2;
use graphics::color;
use graphics::texture;
use graphics::drawable;
use graphics::render_window::RenderWindow;

#[doc(hidden)]
pub mod csfml {
    
    use core::libc::{c_void, c_float, c_uint};
    use system::vector2;
    use graphics::color;
    use graphics::texture;
    use rsfml::sfTypes::sfBool;

    pub struct sfRectangleShape {
        This : *c_void
    }
    
    pub extern "C" {
        fn sfRectangleShape_create() -> *sfRectangleShape;
        fn sfRectangleShape_copy(shape : *sfRectangleShape) -> *sfRectangleShape;
        fn sfRectangleShape_destroy(shape : *sfRectangleShape) -> ();
        fn sfRectangleShape_setPosition(shape : *sfRectangleShape, position : vector2::csfml::sfVector2f) -> ();
        fn sfRectangleShape_setRotation(shape : *sfRectangleShape, angle : c_float) -> ();
        fn sfRectangleShape_setScale(shape : *sfRectangleShape, scale : vector2::csfml::sfVector2f) -> ();
        fn sfRectangleShape_setOrigin(shape : *sfRectangleShape, origin : vector2::csfml::sfVector2f) -> ();
        fn sfRectangleShape_getPosition(shape : *sfRectangleShape) -> vector2::csfml::sfVector2f;
        fn sfRectangleShape_getRotation(shape : *sfRectangleShape) -> c_float;
        fn sfRectangleShape_getScale(shape : *sfRectangleShape) -> vector2::csfml::sfVector2f;
        fn sfRectangleShape_getOrigin(shape : *sfRectangleShape) -> vector2::csfml::sfVector2f;
        fn sfRectangleShape_move(shape : *sfRectangleShape, offset : vector2::csfml::sfVector2f) -> ();
        fn sfRectangleShape_rotate(shape : *sfRectangleShape, angle : c_float) -> ();
        fn sfRectangleShape_scale(shape : *sfRectangleShape, factors : vector2::csfml::sfVector2f) -> ();
        //fn sfRectangleShape_getTransform(shape : *sfRectangleShape) -> sfTransform;
        //fn sfRectangleShape_getInverseTransform(shape : *sfRectangleShape) -> sfTransform;
        fn sfRectangleShape_setTexture(shape : *sfRectangleShape, texture : *texture::csfml::sfTexture, resetRect : sfBool) -> ();
        //fn sfRectangleShape_setTextureRect(shape : *sfRectangleShape, rect : IntRect) -> ();
        fn sfRectangleShape_setFillColor(shape : *sfRectangleShape, color : color::csfml::sfColor) -> ();
        fn sfRectangleShape_setOutlineColor(shape : *sfRectangleShape, color : color::csfml::sfColor) -> ();
        fn sfRectangleShape_setOutlineThickness(shape : *sfRectangleShape, thickness : c_float) -> ();
        fn sfRectangleShape_getTexture(shape : *sfRectangleShape) -> *texture::csfml::sfTexture;
        //fn sfRectangleShape_getTextureRect(shape : *sfRectangleShape) -> sfIntRect;
        fn sfRectangleShape_getFillColor(shape : *sfRectangleShape) -> color::csfml::sfColor;
        fn sfRectangleShape_getOutlineColor(shape : *sfRectangleShape) -> color::csfml::sfColor;
        fn sfRectangleShape_getOutlineThickness(shape : *sfRectangleShape) -> c_float;
        fn sfRectangleShape_getPointCount(shape : *sfRectangleShape) -> c_uint;
        fn sfRectangleShape_getPoint(shape : *sfRectangleShape, index : c_uint) -> vector2::csfml::sfVector2f;
        fn sfRectangleShape_setSize(shape : *sfRectangleShape, size : vector2::csfml::sfVector2f) -> ();
        fn sfRectangleShape_getSize(shape : *sfRectangleShape) -> vector2::csfml::sfVector2f;
        //fn sfRectangleShape_getLocalBounds(shape : *sfRectangleShape) -> sfFloatRect;
        //fn sfRectangleShape_getGlobalBounds(shape : *sfRectangleShape) -> sfFloatRect;
    }
}

#[doc(hidden)]
pub struct RectangleShape {
    priv rectangleShape : *csfml::sfRectangleShape
}

impl RectangleShape {
    pub fn new() -> RectangleShape {
        RectangleShape { rectangleShape : unsafe {csfml::sfRectangleShape_create()} }
    }

    pub fn new_copy(rectangleShape : &RectangleShape) -> RectangleShape {
        RectangleShape {rectangleShape : unsafe {csfml::sfRectangleShape_copy(rectangleShape.unwrap_rectangle_shape())} }
    }
    
    pub fn set_position(&self, position : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfRectangleShape_setPosition(self.rectangleShape, vector2::unwrap_vector2f(position))
        }
    }
    
    pub fn set_rotation(&self, angle : float) -> () {
        unsafe {
            csfml::sfRectangleShape_setRotation(self.rectangleShape, angle as c_float)
        }
    }

    pub fn set_scale(&self, scale : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfRectangleShape_setScale(self.rectangleShape, vector2::unwrap_vector2f(scale))
        }
    }

    pub fn set_origin(&self, origin : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfRectangleShape_setOrigin(self.rectangleShape, vector2::unwrap_vector2f(origin))
        }
    }

    pub fn scale(&self, factors : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfRectangleShape_scale(self.rectangleShape, vector2::unwrap_vector2f(factors))
        }
    }

    pub fn move(&self, offset : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfRectangleShape_move(self.rectangleShape, vector2::unwrap_vector2f(offset))
        }
    }

    pub fn get_rotation(&self) -> float {
        unsafe {
            csfml::sfRectangleShape_getRotation(self.rectangleShape) as float
        }
    }
    
    pub fn rotate(&self, angle : float) -> () {
        unsafe {
            csfml::sfRectangleShape_rotate(self.rectangleShape, angle as c_float)
        }
    }

    pub fn get_position(&self) -> vector2::Vector2f {
        vector2::wrap_vector2f(unsafe { csfml::sfRectangleShape_getPosition(self.rectangleShape) })
    }

    pub fn get_scale(&self) -> vector2::Vector2f {
        vector2::wrap_vector2f(unsafe { csfml::sfRectangleShape_getScale(self.rectangleShape) })
    }

    pub fn get_origin(&self) -> vector2::Vector2f {
        vector2::wrap_vector2f(unsafe { csfml::sfRectangleShape_getOrigin(self.rectangleShape) })
    }

    pub fn get_size(&self) -> vector2::Vector2f {
        vector2::wrap_vector2f(unsafe { csfml::sfRectangleShape_getSize(self.rectangleShape) })
    }

    pub fn get_point(&self, index : uint) -> vector2::Vector2f {
        vector2::wrap_vector2f(unsafe { csfml::sfRectangleShape_getPoint(self.rectangleShape, index as c_uint) })
    }

    pub fn set_size(&self, size : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfRectangleShape_setSize(self.rectangleShape, vector2::unwrap_vector2f(size))
        }
    }

    pub fn set_texture(&self, texture : &texture::Texture, resetRect : bool) -> () {
        match resetRect {
            false       => unsafe { csfml::sfRectangleShape_setTexture(self.rectangleShape, texture.unwrap_texture(), 0) },
            true        => unsafe { csfml::sfRectangleShape_setTexture(self.rectangleShape, texture.unwrap_texture(), 1) }
        }
    }

    pub fn set_fill_color(&self, color : color::Color) -> () {
        unsafe {
            csfml::sfRectangleShape_setFillColor(self.rectangleShape, color.unwrap_color())
        }
    }
    
    pub fn set_outline_color(&self, color : color::Color) -> () {
        unsafe {
            csfml::sfRectangleShape_setOutlineColor(self.rectangleShape, color.unwrap_color())
        }
    }

    pub fn set_outline_thickness(&self, thickness : float) -> () {
        unsafe {
            csfml::sfRectangleShape_setOutlineThickness(self.rectangleShape, thickness as c_float)
        }
    }

    pub fn get_texture(&self) -> texture::Texture {
        unsafe {
            texture::Texture::wrap_texture(csfml::sfRectangleShape_getTexture(self.rectangleShape))
        }
    }

    pub fn get_fill_color(&self) -> color::Color {
        color::Color::wrap_color(unsafe { csfml::sfRectangleShape_getFillColor(self.rectangleShape) })
    }

    pub fn get_outline_color(&self) -> color::Color {
        color::Color::wrap_color(unsafe { csfml::sfRectangleShape_getOutlineColor(self.rectangleShape) })
    }

    pub fn get_outline_thickness(&self) -> float {
        unsafe { csfml::sfRectangleShape_getOutlineThickness(self.rectangleShape) as float }
    }

    pub fn get_point_count(&self) -> uint {
        unsafe {
            csfml::sfRectangleShape_getPointCount(self.rectangleShape) as uint
        }
    }

    #[doc(hidden)]
    pub fn wrap_rectangle_shape(rectangleShape : *csfml::sfRectangleShape) -> RectangleShape {
        RectangleShape { rectangleShape : rectangleShape }
    }
    
    #[doc(hidden)]
    pub fn unwrap_rectangle_shape(&self) -> *csfml::sfRectangleShape {
        self.rectangleShape
    }
}

impl drawable::Drawable for RectangleShape {
    pub fn draw_in_render_window(&self, renderWindow : &RenderWindow) -> () {
        renderWindow.draw_rectangle_shape(self);
    }
}

impl Drop for RectangleShape {
    fn finalize(&self) -> () {
        unsafe {
            csfml::sfRectangleShape_destroy(self.rectangleShape)
        }
    }
}