use raytracing_iow::{render::camera::CameraConfig, vec3::Vec3};
use utoipa::{
    openapi::{ArrayBuilder, ObjectBuilder, OneOfBuilder, Ref, RefOr, Schema, SchemaType},
    OpenApi, ToSchema,
};

use crate::{
    endpoints::{
        self,
        gen::{GenImageRequest, GenImageResponse},
        status::{CompletedImageResponse, ImageStatus},
    },
    models::{Color, Material, Object, Shape, Sphere},
};

pub type ImageStatusResponse = ImageStatus<CompletedImageResponse>;

impl ToSchema<'static> for ImageStatusResponse {
    fn schema() -> (
        &'static str,
        utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    ) {
        (
            "ImageStatusResponse",
            RefOr::T(Schema::OneOf(
                OneOfBuilder::new()
                    .title(Some("ImageStatus".to_string()))
                    .item(
                        ObjectBuilder::new()
                            .title(Some("Queued"))
                            .schema_type(SchemaType::String)
                            .build(),
                    )
                    .item(
                        ArrayBuilder::new()
                            .title(Some("Rendering"))
                            .items(
                                ObjectBuilder::new()
                                    .schema_type(SchemaType::Integer)
                                    .build(),
                            )
                            .build(),
                    )
                    .item(Ref::from_schema_name("CompletedImageResponse"))
                    .example(Some("Queued".into()))
                    .build(),
            )),
        )
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        endpoints::gen::gen_image,
        endpoints::status::image_status,
        endpoints::download::download_image
    ),
    components(schemas(
        GenImageRequest,
        GenImageResponse,
        CompletedImageResponse,
        Color,
        Sphere,
        Shape,
        Material,
        Object,
        Vec3,
        CameraConfig,
        ImageStatusResponse
    ))
)]
pub struct ApiDoc;
