
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse},
    routing,
    Router,
};
use bytes::Bytes;
use serde::Deserialize;

use diesel::{self, RunQueryDsl, QueryableByName, sql_query, sql_types};

use crate::model::{ app_state::AppStateArc};

#[derive(Deserialize)]
pub struct TileParams {
    z: i32,
    x: i32,
    y: i32,
}

#[derive(Debug, QueryableByName)]
pub struct MvtTile {
    #[diesel(sql_type = diesel::sql_types::Bytea)]
    pub mvt: Vec<u8>,
}

pub async fn get_tile(
    State(app_state): State<AppStateArc>,
    Path(TileParams { z, x, y }): Path<TileParams>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut conn = app_state.gis_db_pool.get().unwrap();
    let query = r#"
        SELECT ST_AsMVT(tile, 'basemap', 4096, 'geom') AS mvt
        FROM (
            SELECT
                fid,
                layer,
                ST_AsMVTGeom(geom, ST_TileEnvelope($1, $2, $3), 4096, 64, true) AS geom
            FROM basemap
            WHERE geom && ST_TileEnvelope($1, $2, $3)
        ) AS tile;
    "#;
    let result  = sql_query(query)
        .bind::<sql_types::Integer, _>(z)
        .bind::<sql_types::Integer, _>(x)
        .bind::<sql_types::Integer, _>(y)
        .load::<MvtTile>(&mut conn);
    if let Err(err) = result {
        return Err((StatusCode::OK, err.to_string()));
    }
    let rows = result.unwrap();
    if rows.is_empty() {
        return Err((StatusCode::OK, "is empty".to_string()));
    }

    let pbf: &[u8] = &rows[0].mvt;
    let body = Bytes::copy_from_slice(pbf);

    Ok((
        StatusCode::OK,
        [
            ("Content-Type", "application/x-protobuf"),
            ("Cache-Control", "public, max-age=86400"), // 缓存 1 天
        ],
        body,
    ))
}

pub fn router<'a>() -> Router<AppStateArc> {
    Router::new()
        .route("/tiles/{z}/{x}/{y}/tile.pbf", routing::get(get_tile))
}
