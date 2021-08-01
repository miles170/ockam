use ockam::{route, Context, Entity, Result, SecureChannels, TrustEveryonePolicy, Vault};

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    // Create a Vault to safely store secret keys for Alice and Bob.
    let vault = Vault::create(&ctx)?;

    // Create an Entity to represent Bob.
    let mut bob = Entity::create(&ctx, &vault)?;

    // Create a secure channel listener for Bob that will wait for requests to
    // initiate an Authenticated Key Exchange.
    bob.create_secure_channel_listener("bob", TrustEveryonePolicy)?;

    // Create an entity to represent Alice.
    let mut alice = Entity::create(&ctx, &vault)?;

    // As Alice, connect to Bob's secure channel listener and perform an
    // Authenticated Key Exchange to establish an encrypted secure channel with Bob.
    let channel = alice.create_secure_channel("bob", TrustEveryonePolicy)?;

    // Send a message, ** THROUGH ** the secure channel,
    // to the "app" worker on the other side.
    //
    // This message will automatically get encrypted when it enters the channel
    // and decrypted just before it exits the channel.
    ctx.send(route![channel, "app"], "Hello Ockam!".to_string()).await?;

    // Wait to receive a message for the "app" worker and print it.
    let message = ctx.receive::<String>().await?;
    println!("App Received: {}", message); // should print "Hello Ockam!"

    // Stop all workers, stop the node, cleanup and return.
    ctx.stop().await
}