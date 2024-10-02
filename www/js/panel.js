const times = document.getElementsByClassName("replaceme-time");
const money = document.getElementsByClassName("replaceme-money");
const issuers = document.getElementsByClassName("replaceme-issuer");

const ISSUERS = [
    "MCDONALDS,MLYNY,NITRA",
    "PAYPAL*APPLE,INC",
    "SPOTIFY,LTD",
    "ALZA,CZ,PRAGUE",
    "Revolut**0000*",
    "ZSSK.SK BRATISLAVA",
    "APPLE.COM/BILL ITUNES.COM",
    "DDUbian SK,ZILINA",
    "LIDL,NR",
    "MINIT,CENTRO,NITRA",
    "Microsoft*Xbox"
];

function randInt(min, max) {
    return Math.random() * (max - min + 1) + min;
}

function floatToStr(value, precision) {
    var power = Math.pow(10, precision || 0);
    return String(Math.round(value * power) / power);
}

for (let item of times) {
    const hours = Math.floor(randInt(0, 22));
    const minutes = Math.floor(randInt(0, 58));

    item.textContent = `${String(hours).padStart(2, "0")}:${String(minutes).padStart(2, "0")}`;
}

for (let item of money) {
    const amount = randInt(0, 420.69);
    item.textContent = `${floatToStr(amount, 2)}€`;
}

for (let item of issuers) {
    const index = Math.floor(Math.random() * ISSUERS.length);
    const issuer = ISSUERS[index];
    item.textContent = `${issuer}`;
}