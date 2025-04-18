use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Samsung Galaxy S24".to_string(),
            price: 1149.99,
            description: "The latest Samsung Galaxy S24 offers a stunning AMOLED display, pro-grade camera system, and lightning-fast performance powered by the Snapdragon 8 Gen 3 processor.".to_string(),
            image: "/galaxy_s24.jpeg".to_string()
        },
        Product {
            id: 2,
            name: "Sony WH-1000XM5 Wireless Headphones".to_string(),
            price: 499.99,
            description: "Experience industry-leading noise cancellation and crystal-clear audio with the Sony WH-1000XM5. Perfect for music lovers and remote workers alike.".to_string(),
            image: "/sony_xm5.jpeg".to_string()
        },
        Product {
            id: 3,
            name: "Apple MacBook Air M2".to_string(),
            price: 1399.99,
            description: "Powered by Apple's M2 chip, the MacBook Air delivers exceptional speed, all-day battery life, and a sleek, lightweight design perfect for productivity on the go.".to_string(),
            image: "/macbook_air.jpeg".to_string()
        },
        Product {
            id: 4,
            name: "LG 55\" 4K UHD Smart TV".to_string(),
            price: 649.99,
            description: "Transform your home entertainment with this LG 55-inch 4K UHD Smart TV. Features HDR, built-in streaming apps, and ultra-clear visuals for immersive viewing.".to_string(),
            image: "/lg_tv.jpg".to_string()
        },
        Product {
            id: 5,
            name: "Bose SoundLink Revolve II Bluetooth Speaker".to_string(),
            price: 229.99,
            description: "Enjoy 360° sound and deep bass with the Bose SoundLink Revolve II. A portable, water-resistant Bluetooth speaker perfect for indoor and outdoor use.".to_string(),
            image: "/bose_speaker.jpg".to_string()
        },
        Product {
            id: 6,
            name: "Canon EOS Rebel T8i DSLR Camera".to_string(),
            price: 1049.99,
            description: "Capture stunning photos and 4K videos with the Canon EOS Rebel T8i, ideal for both beginners and enthusiasts with its intuitive controls and advanced autofocus.".to_string(),
            image: "/canon_camera.jpg".to_string()
        },
        Product {
            id: 7,
            name: "ASUS ROG Strix Gaming Laptop".to_string(),
            price: 1899.99,
            description: "Built for gamers, the ASUS ROG Strix packs an NVIDIA RTX GPU and high refresh rate display into a stylish chassis for ultimate performance and style.".to_string(),
            image: "/asus_rog.jpg".to_string()
        },
        Product {
            id: 8,
            name: "Fitbit Charge 6 Fitness Tracker".to_string(),
            price: 219.99,
            description: "Track your workouts, heart rate, sleep, and more with the Fitbit Charge 6. Includes Google Wallet and GPS for a smarter, healthier lifestyle.".to_string(),
            image: "/fitbit_charge.jpg".to_string()
        },
        Product {
            id: 9,
            name: "Anker PowerCore 20K Power Bank".to_string(),
            price: 59.99,
            description: "Keep your devices charged with the Anker PowerCore 20,000mAh portable power bank. Features fast charging and dual USB ports for convenience on the go.".to_string(),
            image: "/anker_powerbank.jpeg".to_string()
        },
        Product {
            id: 10,
            name: "Google Nest Thermostat".to_string(),
            price: 179.99,
            description: "Save energy and control your home temperature from anywhere with the Google Nest Thermostat. Smart scheduling and voice control included.".to_string(),
            image: "/nest_thermostat.jpg".to_string()
        }
    ]
}