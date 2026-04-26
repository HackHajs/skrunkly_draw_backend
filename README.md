#  Skrunkly Draw Backend
![rust-version](https://img.shields.io/badge/1.95-a?style=for-the-badge&logo=rust&logoColor=%23ffffff&label=rust&labelColor=%23f46623&color=%23555555&link=https%3A%2F%2Fblog.rust-lang.org%2F2024%2F05%2F02%2FRust-1.78.0.html) ![nix-version](https://img.shields.io/badge/25.11-a?style=for-the-badge&logo=nixos&logoColor=%23ffffff&label=Nix&labelColor=%237bb6e1&color=%23555555&link=https%3A%2F%2Fnixos.org%2F)

## Dependencies
 - [Nix](https://nixos.org/download/)

## Endpoints
### Post
- `GET /v0/post?id=<uuid>`
  **Returns:**
  - `200`
    **Response body:**
    ```json
    {
      "_id": "<uuid>",
      "user": "<uuid>",
      "created_at": "<iso_6801_utc_datetime>"
      "reply": {
        "parent": "<uuid>",
        "on_feed": bool,
      } // This is skipped if the post is not a reply
      "mature": bool,
      "liked_by": ["<uuid>"],
      "flagged_by": ["<uuid>"],
      "skrunkle": {
        "palette": ["str"]; // Exactly 8 elements
        "bg_color": "str",
        "strokes": [
          {
            "color": number,
            "shape": [
               [num, num, num] // Exactly 3 elements  
            ]
          }
        ]
      }
    }
    ```
  - `400` When the uuid in the query is malformed
  - `401`
    **Response body:**
    ```json
    {
      "type": { "authentication": "<type>" } // type is one of https://github.com/HackHajs/skrunkly_draw_backend/blob/trunk/src/error.rs#L25
      "message": "<human_readable_message>"
    }
    ```
  - `500`
    **Response body:**
    ```json
    {
      "type": { "database": "<type>" } // type is one of https://github.com/HackHajs/skrunkly_draw_backend/blob/trunk/src/error.rs#L42
      "message": "<human_readable_message>"
    }
    ```
  
- `GET /v0/post/all`
  - Everything is the same as `GET /v0/post` but you get a list instead. The list is ordered from newest to oldest, and limited to 100 elements. You don't get replies on this list unless the author of the reply specifically wanted the reply to also be published to the feed.

- `GET /v0/post/replies?id=<uuid>`
  - Similar to `/v0/post/all`, but instead you only get posts which reply to the post provided on the query. The skrunkles in each post will contain the strokes of the original post and the strokes added by the reply.

- `POST /v0/post`
  **HTTP Headers**
  - `Content-Type: application/json`
  - `Authorization: Bearer <jwt>`
  **Body:**
  ```json
  {
    "reply": {
      "parent": "<uuid>",
      "on_feed": bool,
    } // Can be skipped if the post shouldn't be a reply
    "mature": bool,
    "skrunkle": {
      "palette": ["str"]; // Exactly 8 elements
      "bg_color": "str",
      "strokes": [
          {
            "color": number,
            "shape": [
              [num, num, num] // Exactly 3 elements  
            ]
          q}
        ]
      }
  }
  ```
  **Returns:**
  - `200` (no body)
  - `400` When the json sent is malformed
  - `401`
    **Response body:**
    ```json
    {
      "type": { "authentication": "<type>" } // type is one of https://github.com/HackHajs/skrunkly_draw_backend/blob/trunk/src/error.rs#L25
      "message": "<human_readable_message>"
    }
  - `500`
    **Response body:**
    ```json
    {
      "type": { "database": "<type>" } // type is one of https://github.com/HackHajs/skrunkly_draw_backend/blob/trunk/src/error.rs#L42
      "message": "<human_readable_message>"
    }
    ```

- `DELETE /v0/post?id=<uuid>`
  **HTTP Headers**
  - `Authorization: Bearer <jwt>`
  **Returns:**
  - `200`: The body is simply the number of updated documents. Should be equal to 1,
  - `400` When the uuid in the query is malformed
  - `401`
    **Response body:**
    ```json
    {
      "type": { "authentication": "<type>" } // type is one of https://github.com/HackHajs/skrunkly_draw_backend/blob/trunk/src/error.rs#L25
      "message": "<human_readable_message>"
    }
  - `500`
    **Response body:**
    ```json
    {
      "type": { "database": "<type>" } // type is one of https://github.com/HackHajs/skrunkly_draw_backend/blob/trunk/src/error.rs#L42
      "message": "<human_readable_message>"
    }
    ```
 
 ### User
 - `GET /user?id=<uuid>`
    **Returns:**
    - `200`
      **Response body:**
      ```json
      {
        "_id": "<uuid>",
        "created_at": "<iso_6801_utc_datetime>",
        "name": "<string>",
        "link": "<string>", // This could be like a personal website or a portfolio
        "profile_picture": "<uuid>", // The profile picture can reference one of the user's posts
      }
      ```
    - `400` When the uuid in the query is malformed
    - `500`
    **Response body:**
    ```json
    {
      "type": { "database": "<type>" } // type is one of https://github.com/HackHajs/skrunkly_draw_backend/blob/trunk/src/error.rs#L42
      "message": "<human_readable_message>"
    }
    ```

- `PUT /user`
  **HTTP Headers**
  - `Content-Type: application/json`
  - `Authorization: Bearer <jwt>`
  **Body:**
  ```json
  {
    "name": "<string>", // All fields are required on create. Fields skipped when updating will stay unchanged.
    "link": "<string>", // Creates the user if it doesn't exist. Updates it otherwise.
    "profile_picture": "<uuid>"
  }
  ```
  **Returns:**
  - `200` (no body)
  - `400` When the json sent is malformed
  - `401`
    **Response body:**
    ```json
    {
      "type": { "authentication": "<type>" } // type is one of https://github.com/HackHajs/skrunkly_draw_backend/blob/trunk/src/error.rs#L25
      "message": "<human_readable_message>"
    }
  - `500`
    **Response body:**
    ```json
    {
      "type": { "database": "<type>" } // type is one of https://github.com/HackHajs/skrunkly_draw_backend/blob/trunk/src/error.rs#L42
      "message": "<human_readable_message>"
    }
    ```

- `DELETE /v0/user?id=<uuid>`
  **HTTP Headers**
  - `Authorization: Bearer <jwt>`
  **Returns:**
  - `200`: The body is simply the number of updated documents. Should be equal to 1,
  - `400` When the uuid in the query is malformed
  - `401`
    **Response body:**
    ```json
    {
      "type": { "authentication": "<type>" } // type is one of https://github.com/HackHajs/skrunkly_draw_backend/blob/trunk/src/error.rs#L25
      "message": "<human_readable_message>"
    }
  - `500`
    **Response body:**
    ```json
    {
      "type": { "database": "<type>" } // type is one of https://github.com/HackHajs/skrunkly_draw_backend/blob/trunk/src/error.rs#L42
      "message": "<human_readable_message>"
    }
    ```
