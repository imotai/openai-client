use openai_dive::v1::api::Client;
use openai_dive::v1::models::FlagshipModel;
use openai_dive::v1::resources::chat::{
    ChatCompletionParametersBuilder, ChatMessage, ChatMessageContent, ChatMessageContentPart,
    ChatMessageImageContentPart, ChatMessageTextContentPart, ImageUrlType,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new_from_env();

    let parameters = ChatCompletionParametersBuilder::default()
        .model(FlagshipModel::Gpt4O.to_string())
        .messages(vec![
            ChatMessage::User {
                content: ChatMessageContent::Text("What is in this image?".to_string()),
                name: None,
            },
            ChatMessage::User {
                content: ChatMessageContent::ContentPart(vec![ChatMessageContentPart::Image(
                    ChatMessageImageContentPart {
                        r#type: "image_url".to_string(),
                        image_url: ImageUrlType {
                            url:
                                "https://images.unsplash.com/photo-1526682847805-721837c3f83b?w=640"
                                    .to_string(),
                            detail: None,
                        },
                    },
                )]),
                name: None,
            },
        ])
        .max_completion_tokens(50u32)
        .build()?;

    let result = client.chat().create(parameters).await?;

    println!("{result:#?}");

    let parameters = ChatCompletionParametersBuilder::default()
        .model("gpt-4o-mini".to_string())
        .messages(vec![
            ChatMessage::System {
                content: ChatMessageContent::Text("You are a friendly assistant".to_string()),
                name: None,
            },
            ChatMessage::User {
                content: ChatMessageContent::ContentPart(vec![
                    ChatMessageContentPart::Image(
                        ChatMessageImageContentPart {
                            r#type: "image_url".to_string(),
                            image_url: ImageUrlType {
                                url:
                                    "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAYAAADgdz34AAAABHNCSVQICAgIfAhkiAAAAAlwSFlzAAAApgAAAKYB3X3/OAAAABl0RVh0U29mdHdhcmUAd3d3Lmlua3NjYXBlLm9yZ5vuPBoAAANCSURBVEiJtZZPbBtFFMZ/M7ubXdtdb1xSFyeilBapySVU8h8OoFaooFSqiihIVIpQBKci6KEg9Q6H9kovIHoCIVQJJCKE1ENFjnAgcaSGC6rEnxBwA04Tx43t2FnvDAfjkNibxgHxnWb2e/u992bee7tCa00YFsffekFY+nUzFtjW0LrvjRXrCDIAaPLlW0nHL0SsZtVoaF98mLrx3pdhOqLtYPHChahZcYYO7KvPFxvRl5XPp1sN3adWiD1ZAqD6XYK1b/dvE5IWryTt2udLFedwc1+9kLp+vbbpoDh+6TklxBeAi9TL0taeWpdmZzQDry0AcO+jQ12RyohqqoYoo8RDwJrU+qXkjWtfi8Xxt58BdQuwQs9qC/afLwCw8tnQbqYAPsgxE1S6F3EAIXux2oQFKm0ihMsOF71dHYx+f3NND68ghCu1YIoePPQN1pGRABkJ6Bus96CutRZMydTl+TvuiRW1m3n0eDl0vRPcEysqdXn+jsQPsrHMquGeXEaY4Yk4wxWcY5V/9scqOMOVUFthatyTy8QyqwZ+kDURKoMWxNKr2EeqVKcTNOajqKoBgOE28U4tdQl5p5bwCw7BWquaZSzAPlwjlithJtp3pTImSqQRrb2Z8PHGigD4RZuNX6JYj6wj7O4TFLbCO/Mn/m8R+h6rYSUb3ekokRY6f/YukArN979jcW+V/S8g0eT/N3VN3kTqWbQ428m9/8k0P/1aIhF36PccEl6EhOcAUCrXKZXXWS3XKd2vc/TRBG9O5ELC17MmWubD2nKhUKZa26Ba2+D3P+4/MNCFwg59oWVeYhkzgN/JDR8deKBoD7Y+ljEjGZ0sosXVTvbc6RHirr2reNy1OXd6pJsQ+gqjk8VWFYmHrwBzW/n+uMPFiRwHB2I7ih8ciHFxIkd/3Omk5tCDV1t+2nNu5sxxpDFNx+huNhVT3/zMDz8usXC3ddaHBj1GHj/As08fwTS7Kt1HBTmyN29vdwAw+/wbwLVOJ3uAD1wi/dUH7Qei66PfyuRj4Ik9is+hglfbkbfR3cnZm7chlUWLdwmprtCohX4HUtlOcQjLYCu+fzGJH2QRKvP3UNz8bWk1qMxjGTOMThZ3kvgLI5AzFfo379UAAAAASUVORK5CYII=".to_string(),
                                detail: None,
                            },
                        },
                    ),
                    ChatMessageContentPart::Text(
                        ChatMessageTextContentPart {
                            r#type: "text".to_string(),
                            text: "Describe this image in Vietnamese.".to_string(),
                        },
                    ),
                ]),
                name: None,
            },
        ])
        .max_completion_tokens(50u32)
        .build()?;

    let result = client.chat().create(parameters).await.unwrap();

    println!("{result:#?}");

    Ok(())
}
