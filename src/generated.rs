use wundergraph::query_builder::types::{HasMany, HasOne};
use wundergraph::scalar::WundergraphScalarValue;
use wundergraph::WundergraphEntity;
use serde_json::Value as JsonValue;

table! {
    cinemas (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    cinemas_movies (id) {
        id -> Int4,
        exposed_format -> Nullable<Int4>,
        pixels_box -> Nullable<Array<Float8>>,
        cinema_id -> Int4,
        movie_id -> Int4,
    }
}

table! {
    color_movie_colormap (id) {
        id -> Int4,
        color_movie_id -> Int4,
        colormap_id -> Int4,
    }
}

table! {
    color_movies (id) {
        id -> Int4,
        format -> Int2,
        default_colormap -> Int4,
    }
}

table! {
    colormaps (id) {
        id -> Int4,
        name -> Text,
        colors -> Array<Text>,
        positions -> Array<Float8>,
    }
}

table! {
    images (id) {
        id -> Int4,
        time -> Timestamp,
        path -> Text,
        #[sql_name = "box"]
        box_ -> Nullable<Array<Float8>>,
        color_movie_id -> Int4,
    }
}

table! {
    images_tags_values (id) {
        id -> Int4,
        tags_value_id -> Int4,
        image_id -> Int4,
    }
}

table! {
    movies (id) {
        id -> Int4,
        identifier -> Text,
        name -> Text,
        description -> Nullable<Text>,
        pixels_box -> Nullable<Array<Float8>>,
        path -> Nullable<Text>,
    }
}

table! {
    movies_tags (id) {
        id -> Int4,
        movie_id -> Int4,
        tag_id -> Int4,
    }
}

table! {
    tags (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    tags_values (id) {
        id -> Int4,
        value -> Text,
        tag_id -> Int4,
    }
}

table! {
    vector_data (id) {
        id -> Int4,
        time -> Timestamp,
        properties -> Nullable<Jsonb>,
        vector_movie_id -> Int4,
    }
}

table! {
    vector_movies (id) {
        id -> Int4,
        default_style -> Nullable<Int4>,
    }
}

table! {
    vector_styles (id) {
        id -> Int4,
        name -> Text,
        style -> Jsonb,
    }
}

table! {
    vector_styles_vector_movies (id) {
        id -> Int4,
        vector_movie_id -> Int4,
        vector_style_id -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    cinemas,
    cinemas_movies,
    color_movie_colormap,
    color_movies,
    colormaps,
    images,
    images_tags_values,
    movies,
    movies_tags,
    tags,
    tags_values,
    vector_data,
    vector_movies,
    vector_styles,
    vector_styles_vector_movies,
);


#[derive(Clone, Debug, Identifiable, WundergraphEntity)]
#[table_name = "cinemas"]
#[primary_key(id)]
pub struct Cinema {
    id: i32,
    name: String,
    cinemas_movies: HasMany<CinemasMovie, cinemas_movies::cinema_id>,
}

#[derive(Clone, Debug, Identifiable, WundergraphEntity)]
#[table_name = "cinemas_movies"]
#[primary_key(id)]
pub struct CinemasMovie {
    id: i32,
    exposed_format: Option<i32>,
    pixels_box: Option<Vec<f64>>,
    cinema_id: HasOne<i32, Cinema>,
    movie_id: HasOne<i32, Movie>,
}

#[derive(Clone, Debug, Identifiable, WundergraphEntity)]
#[table_name = "color_movie_colormap"]
#[primary_key(id)]
pub struct ColorMovieColormap {
    id: i32,
    color_movie_id: HasOne<i32, ColorMovie>,
    colormap_id: HasOne<i32, Colormap>,
}

#[derive(Clone, Debug, Identifiable, WundergraphEntity)]
#[table_name = "color_movies"]
#[primary_key(id)]
pub struct ColorMovie {
    id: i32,
    format: i16,
    default_colormap: HasOne<i32, Colormap>,
    color_movie_colormap: HasMany<ColorMovieColormap, color_movie_colormap::color_movie_id>,
    images: HasMany<Image, images::color_movie_id>,
}

#[derive(Clone, Debug, Identifiable, WundergraphEntity)]
#[table_name = "colormaps"]
#[primary_key(id)]
pub struct Colormap {
    id: i32,
    name: String,
    colors: Vec<String>,
    positions: Vec<f64>,
    color_movie_colormap: HasMany<ColorMovieColormap, color_movie_colormap::colormap_id>,
    color_movies: HasMany<ColorMovie, color_movies::default_colormap>,
}

#[derive(Clone, Debug, Identifiable, WundergraphEntity)]
#[table_name = "images"]
#[primary_key(id)]
pub struct Image {
    id: i32,
    time: chrono::naive::NaiveDateTime,
    path: String,
    box_: Option<Vec<f64>>,
    color_movie_id: HasOne<i32, ColorMovie>,
    images_tags_values: HasMany<ImagesTagsValue, images_tags_values::image_id>,
}

#[derive(Clone, Debug, Identifiable, WundergraphEntity)]
#[table_name = "images_tags_values"]
#[primary_key(id)]
pub struct ImagesTagsValue {
    id: i32,
    tags_value_id: HasOne<i32, TagsValue>,
    image_id: HasOne<i32, Image>,
}

#[derive(Clone, Debug, Identifiable, WundergraphEntity)]
#[table_name = "movies"]
#[primary_key(id)]
pub struct Movie {
    id: i32,
    identifier: String,
    name: String,
    description: Option<String>,
    pixels_box: Option<Vec<f64>>,
    path: Option<String>,
    cinemas_movies: HasMany<CinemasMovie, cinemas_movies::movie_id>,
    movies_tags: HasMany<MoviesTag, movies_tags::movie_id>,
}

#[derive(Clone, Debug, Identifiable, WundergraphEntity)]
#[table_name = "movies_tags"]
#[primary_key(id)]
pub struct MoviesTag {
    id: i32,
    movie_id: HasOne<i32, Movie>,
    tag_id: HasOne<i32, Tag>,
}

#[derive(Clone, Debug, Identifiable, WundergraphEntity)]
#[table_name = "tags"]
#[primary_key(id)]
pub struct Tag {
    id: i32,
    name: String,
    movies_tags: HasMany<MoviesTag, movies_tags::tag_id>,
    tags_values: HasMany<TagsValue, tags_values::tag_id>,
}

#[derive(Clone, Debug, Identifiable, WundergraphEntity)]
#[table_name = "tags_values"]
#[primary_key(id)]
pub struct TagsValue {
    id: i32,
    value: String,
    tag_id: HasOne<i32, Tag>,
    images_tags_values: HasMany<ImagesTagsValue, images_tags_values::tags_value_id>,
}

#[derive(Clone, Debug, Identifiable, WundergraphEntity)]
#[table_name = "vector_data"]
#[primary_key(id)]
pub struct VectorData {
    id: i32,
    time: chrono::naive::NaiveDateTime,
    properties: Option<JsonValue>,
    vector_movie_id: HasOne<i32, VectorMovie>,
}

#[derive(Clone, Debug, Identifiable, WundergraphEntity)]
#[table_name = "vector_movies"]
#[primary_key(id)]
pub struct VectorMovie {
    id: i32,
    default_style: Option<HasOne<i32, VectorStyle>>,
    vector_data: HasMany<VectorData, vector_data::vector_movie_id>,
    vector_styles_vector_movies: HasMany<VectorStylesVectorMovie, vector_styles_vector_movies::vector_movie_id>,
}

#[derive(Clone, Debug, Identifiable, WundergraphEntity)]
#[table_name = "vector_styles"]
#[primary_key(id)]
pub struct VectorStyle {
    id: i32,
    name: String,
    style: JsonValue,
    vector_movies: HasMany<VectorMovie, vector_movies::default_style>,
    vector_styles_vector_movies: HasMany<VectorStylesVectorMovie, vector_styles_vector_movies::vector_style_id>,
}

#[derive(Clone, Debug, Identifiable, WundergraphEntity)]
#[table_name = "vector_styles_vector_movies"]
#[primary_key(id)]
pub struct VectorStylesVectorMovie {
    id: i32,
    vector_movie_id: HasOne<i32, VectorMovie>,
    vector_style_id: HasOne<i32, VectorStyle>,
}



wundergraph::query_object!{
    Query {
        Cinema,
        CinemasMovie,
        ColorMovieColormap,
        ColorMovie,
        Colormap,
        Image,
        ImagesTagsValue,
        Movie,
        MoviesTag,
        Tag,
        TagsValue,
        VectorData,
        VectorMovie,
        VectorStyle,
        VectorStylesVectorMovie,
    }
}


#[derive(Insertable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "cinemas"]
pub struct NewCinema {
    name: String,
}

#[derive(AsChangeset, Identifiable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "cinemas"]
#[primary_key(id)]
pub struct CinemaChangeset {
    id: i32,
    name: String,
}

#[derive(Insertable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "cinemas_movies"]
pub struct NewCinemasMovie {
    exposed_format: Option<i32>,
    pixels_box: Option<Vec<f64>>,
    cinema_id: i32,
    movie_id: i32,
}

#[derive(AsChangeset, Identifiable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "cinemas_movies"]
#[primary_key(id)]
pub struct CinemasMovieChangeset {
    id: i32,
    exposed_format: Option<i32>,
    pixels_box: Option<Vec<f64>>,
    cinema_id: i32,
    movie_id: i32,
}

#[derive(Insertable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "color_movie_colormap"]
pub struct NewColorMovieColormap {
    color_movie_id: i32,
    colormap_id: i32,
}

#[derive(AsChangeset, Identifiable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "color_movie_colormap"]
#[primary_key(id)]
pub struct ColorMovieColormapChangeset {
    id: i32,
    color_movie_id: i32,
    colormap_id: i32,
}

#[derive(Insertable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "color_movies"]
pub struct NewColorMovie {
    format: i16,
    default_colormap: i32,
}

#[derive(AsChangeset, Identifiable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "color_movies"]
#[primary_key(id)]
pub struct ColorMovieChangeset {
    id: i32,
    format: i16,
    default_colormap: i32,
}

#[derive(Insertable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "colormaps"]
pub struct NewColormap {
    name: String,
    colors: Vec<String>,
    positions: Vec<f64>,
}

#[derive(AsChangeset, Identifiable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "colormaps"]
#[primary_key(id)]
pub struct ColormapChangeset {
    id: i32,
    name: String,
    colors: Vec<String>,
    positions: Vec<f64>,
}

#[derive(Insertable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "images"]
pub struct NewImage {
    time: chrono::naive::NaiveDateTime,
    path: String,
    box_: Option<Vec<f64>>,
    color_movie_id: i32,
}

#[derive(AsChangeset, Identifiable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "images"]
#[primary_key(id)]
pub struct ImageChangeset {
    id: i32,
    time: chrono::naive::NaiveDateTime,
    path: String,
    box_: Option<Vec<f64>>,
    color_movie_id: i32,
}

#[derive(Insertable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "images_tags_values"]
pub struct NewImagesTagsValue {
    tags_value_id: i32,
    image_id: i32,
}

#[derive(AsChangeset, Identifiable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "images_tags_values"]
#[primary_key(id)]
pub struct ImagesTagsValueChangeset {
    id: i32,
    tags_value_id: i32,
    image_id: i32,
}

#[derive(Insertable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "movies"]
pub struct NewMovie {
    identifier: String,
    name: String,
    description: Option<String>,
    pixels_box: Option<Vec<f64>>,
    path: Option<String>,
}

#[derive(AsChangeset, Identifiable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "movies"]
#[primary_key(id)]
pub struct MovieChangeset {
    id: i32,
    identifier: String,
    name: String,
    description: Option<String>,
    pixels_box: Option<Vec<f64>>,
    path: Option<String>,
}

#[derive(Insertable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "movies_tags"]
pub struct NewMoviesTag {
    movie_id: i32,
    tag_id: i32,
}

#[derive(AsChangeset, Identifiable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "movies_tags"]
#[primary_key(id)]
pub struct MoviesTagChangeset {
    id: i32,
    movie_id: i32,
    tag_id: i32,
}

#[derive(Insertable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "tags"]
pub struct NewTag {
    name: String,
}

#[derive(AsChangeset, Identifiable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "tags"]
#[primary_key(id)]
pub struct TagChangeset {
    id: i32,
    name: String,
}

#[derive(Insertable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "tags_values"]
pub struct NewTagsValue {
    value: String,
    tag_id: i32,
}

#[derive(AsChangeset, Identifiable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "tags_values"]
#[primary_key(id)]
pub struct TagsValueChangeset {
    id: i32,
    value: String,
    tag_id: i32,
}

#[derive(Insertable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "vector_data"]
pub struct NewVectorData {
    time: chrono::naive::NaiveDateTime,
    properties: Option<JsonValue>,
    vector_movie_id: i32,
}

#[derive(AsChangeset, Identifiable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "vector_data"]
#[primary_key(id)]
pub struct VectorDataChangeset {
    id: i32,
    time: chrono::naive::NaiveDateTime,
    properties: Option<JsonValue>,
    vector_movie_id: i32,
}

#[derive(Insertable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "vector_movies"]
pub struct NewVectorMovie {
    default_style: Option<i32>,
}

#[derive(AsChangeset, Identifiable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "vector_movies"]
#[primary_key(id)]
pub struct VectorMovieChangeset {
    id: i32,
    default_style: Option<i32>,
}

#[derive(Insertable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "vector_styles"]
pub struct NewVectorStyle {
    name: String,
    style: JsonValue,
}

#[derive(AsChangeset, Identifiable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "vector_styles"]
#[primary_key(id)]
pub struct VectorStyleChangeset {
    id: i32,
    name: String,
    style: JsonValue,
}

#[derive(Insertable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "vector_styles_vector_movies"]
pub struct NewVectorStylesVectorMovie {
    vector_movie_id: i32,
    vector_style_id: i32,
}

#[derive(AsChangeset, Identifiable, juniper::GraphQLInputObject, Clone, Debug)]
#[graphql(scalar = "WundergraphScalarValue")]
#[table_name = "vector_styles_vector_movies"]
#[primary_key(id)]
pub struct VectorStylesVectorMovieChangeset {
    id: i32,
    vector_movie_id: i32,
    vector_style_id: i32,
}

wundergraph::mutation_object!{
    Mutation{
        Cinema(insert = NewCinema, update = CinemaChangeset, ),
        CinemasMovie(insert = NewCinemasMovie, update = CinemasMovieChangeset, ),
        ColorMovieColormap(insert = NewColorMovieColormap, update = ColorMovieColormapChangeset, ),
        ColorMovie(insert = NewColorMovie, update = ColorMovieChangeset, ),
        Colormap(insert = NewColormap, update = ColormapChangeset, ),
        Image(insert = NewImage, update = ImageChangeset, ),
        ImagesTagsValue(insert = NewImagesTagsValue, update = ImagesTagsValueChangeset, ),
        Movie(insert = NewMovie, update = MovieChangeset, ),
        MoviesTag(insert = NewMoviesTag, update = MoviesTagChangeset, ),
        Tag(insert = NewTag, update = TagChangeset, ),
        TagsValue(insert = NewTagsValue, update = TagsValueChangeset, ),
        VectorData(insert = NewVectorData, update = VectorDataChangeset, ),
        VectorMovie(insert = NewVectorMovie, update = VectorMovieChangeset, ),
        VectorStyle(insert = NewVectorStyle, update = VectorStyleChangeset, ),
        VectorStylesVectorMovie(insert = NewVectorStylesVectorMovie, update = VectorStylesVectorMovieChangeset, ),
    }
}

