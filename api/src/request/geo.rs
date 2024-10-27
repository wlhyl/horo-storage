use serde::Deserialize;
use validator::{Validate, ValidationError};

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

/// insert/update tag
#[cfg_attr(feature = "swagger", derive(ToSchema))]
#[derive(Deserialize, Validate)]
// skip_on_field_errors值为true时，字段验证错误，不会调用此函数
#[validate(schema(function = "validate_long_lat", skip_on_field_errors = true))]
pub struct GeoRequest {
    /// 城市名
    #[validate(length(min = 1), non_control_character)]
    pub name: String,
    /// 东:+，西:-
    pub east: bool,
    /// 地理经度
    #[validate(range(min = 0, max = 180, message = "-180<=地理经度的度<=180"))]
    pub long_d: u16,
    #[validate(range(min = 0, max = 59, message = "-59<=地理经度的分<=59"))]
    pub long_m: u8,
    #[validate(range(min = 0, max = 59, message = "-59<=地理经度的秒<=59"))]
    pub long_s: u8,

    /// 北:+, 南:-
    pub north: bool,

    /// 地理纬度
    #[validate(range(min = 0, max = 90, message = "-90<=地理经度的度<=90"))]
    pub lat_d: u8,
    #[validate(range(min = 0, max = 59, message = "-59<=地理经度的分<=59"))]
    pub lat_m: u8,
    #[validate(range(min = 0, max = 59, message = "-59<=地理经度的秒<=59"))]
    pub lat_s: u8,
}

fn validate_long_lat(geo: &GeoRequest) -> Result<(), ValidationError> {
    // if geo.long_d == 180 && (geo.long_m > 0 || geo.long_s > 0) {
    //     return Err(ValidationError::new("long最大值为180."));
    // }
    // if geo.lat_d == 90 && (geo.lat_m > 0 || geo.lat_s > 0) {
    //     return Err(ValidationError::new("lat最大值为90."));
    // }

    // 通过计算比较大小是可以的，因为u32::MAX > u16::MAX*3600 + u8::MAX*60 + u8::MAX
    let long = u32::from(geo.long_d) * 3600 + u32::from(geo.long_m) * 60 + u32::from(geo.long_s);
    if long > 180 * 3600 {
        return Err(ValidationError::new("long最大值为180."));
    }

    let lat = u32::from(geo.lat_d) * 3600 + u32::from(geo.lat_m) * 60 + u32::from(geo.lat_s);
    if lat > 90 * 3600 {
        return Err(ValidationError::new("lat最大值为90."));
    }
    Ok(())
}