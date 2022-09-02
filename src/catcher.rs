use rocket_contrib::json::JsonValue;

#[catch(404)]
pub fn not_found() -> JsonValue {
    return json!({
        "status": "404",
        "reason": "Resource was not found."
    });
}
