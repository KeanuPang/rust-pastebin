use rocket_contrib::json::JsonValue;

#[catch(404)]
pub fn not_found() -> JsonValue {
    return json!({
        "status": "error",
        "reason": "Resource was not found."
    });
}
