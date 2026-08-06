<div dir="rtl" align="center">
  <img src="assets/logo.png" alt="لوگوی دوسو" width="150" height="150">
  <h1>دوسو (Dosu)</h1>
  <p>یک رپر مدرن و کراس‌پلتفرم برای ترمینال با پشتیبانی از متن دوزبانه — پروژه‌ای از راست‌نگار</p>
  <br>
  <p>
    <img src="https://img.shields.io/badge/rust-+v1.7-orange?style=for-the-badge&logo=rust&logoColor=white" alt="راست">
    <img src="https://img.shields.io/badge/version-0.1.0-22C8E6?style=for-the-badge" alt="نسخه">
    <img src="https://img.shields.io/badge/License-MIT-blue?style=for-the-badge" alt="مجوز">
    <img src="https://img.shields.io/badge/Platform-Linux%20%7C%20macOS-lightgrey?style=for-the-badge" alt="سکوی اجرا">
  </p>
  <br>
  <p>
    <a href="README.md" dir="ltr">English</a> •
    <b>فارسی</b> •
    <a href="README_AR.md">العربية</a> •
    <a href="README_HE.md">עברית</a>
  </p>
  <br>
  <table>
    <tr>
      <td align="center" width="110"><a href="#درباره"><img src="assets/icons/about.svg" width="44" alt="درباره"/><br/><sub><b>درباره</b></sub></a></td>
      <td align="center" width="110"><a href="#نصب"><img src="assets/icons/install.svg" width="44" alt="نصب"/><br/><sub><b>نصب</b></sub></a></td>
      <td align="center" width="110"><a href="#حمایت-از-پروژه"><img src="assets/icons/donate.svg" width="44" alt="دونیت"/><br/><sub><b>دونیت</b></sub></a></td>
      <td align="center" width="110"><a href="#استفاده"><img src="assets/icons/usage.svg" width="44" alt="استفاده"/><br/><sub><b>استفاده</b></sub></a></td>
      <td align="center" width="110"><a href="#پیکربندی"><img src="assets/icons/config.svg" width="44" alt="پیکربندی"/><br/><sub><b>پیکربندی</b></sub></a></td>
      <td align="center" width="110"><a href="#مشکلات-شناخته‌شده"><img src="assets/icons/issues.svg" width="44" alt="مشکلات"/><br/><sub><b>مشکلات</b></sub></a></td>
      <td align="center" width="110"><a href="#تماس"><img src="assets/icons/contact.svg" width="44" alt="تماس"/><br/><sub><b>تماس</b></sub></a></td>
    </tr>
  </table>
</div>

<br>

<div dir="rtl">

## درباره

**دوسو** یک رپر پیشرفته برای ترمینال است که به‌طور ویژه برای پردازش و نمایش متن‌های دوزبانه (Bidirectional) طراحی شده است. این پروژه با زبان Rust و بر پایه [`dosu-core`](https://github.com/RustNegar/dosu-core) ساخته شده و چالش‌های پیچیدهٔ نمایش زبان‌های راست‌به‌چپ مانند فارسی و عربی را در محیط ترمینال حل می‌کند، بدون اینکه ابزارهایی که همین حالا استفاده می‌کنید را به‌هم بریزد.

<br>

## نصب

### نصب سریع (لینوکس/مک‌او‌اس)

```bash
curl -fsSL https://raw.githubusercontent.com/RustNegar/dosu/main/install.sh | sh
```

### هوم‌برو (مک‌او‌اس)

```bash
brew install rustnegar/dosu/dosu
```

### ساخت از سورس

```bash
# کلون کردن مخزن
git clone https://github.com/RustNegar/dosu.git
cd dosu

# ساخت در حالت Release
cargo build --release

# نصب باینری
cargo install --path .
```

### پیش‌نیازها

- Rust نسخه ۱.۷۰ یا بالاتر (فقط برای ساخت از سورس لازم است)
- یک شبیه‌ساز ترمینال سازگار (Kitty، iTerm2، Ghostty، WezTerm و غیره)
- سیستم‌عامل شبه‌یونیکس (لینوکس یا مک‌او‌اس)

<br>

## استفاده

### استفاده پایه

برای شروع یک نشست ترمینال دوزبانه جدید، کافیست دستور زیر را اجرا کنید:

```bash
dosu
```

### دستور تشخیصی

ابزار تشخیصی داخلی را برای بررسی محیط خود اجرا کنید:

```bash
dosu doctor
```

این دستور موارد زیر را بررسی می‌کند:

- سازگاری ترمینال
- تداخل‌های پیکربندی شناخته‌شده (Kitty، tmux، vi-mode)
- تنظیم متغیرهای محیطی
- راه‌حل‌های پیشنهادی

### حالت اشکال‌زدایی

برای توسعه، لاگ‌گیری دقیق را فعال کنید:

```bash
export DOSU_DEBUG_DIR=/tmp/dosu-debug
dosu
```

این دستور سه فایل لاگ ایجاد می‌کند:

| فایل                   | محتوا                        |
| ---------------------- | ---------------------------- |
| `child_to_dosu.log`    | بایت‌های خام از فرآیند فرزند |
| `dosu_to_child.log`    | بایت‌های نوشته‌شده به فرزند  |
| `dosu_to_terminal.log` | خروجی رندر به ترمینال        |

### گزینه‌های خط فرمان

```bash
dosu --help
```

<br>

## پیکربندی

دوسو به‌صورت پیش‌فرض با تنظیمات منطقی کار می‌کند. با این حال، ممکن است نیاز باشد تنظیمات ترمینال یا شل خود را برای تجربهٔ بهینه تغییر دهید.

### تنظیمات پیشنهادی ترمینال

- **فونت** — از فونتی با پشتیبانی خوب از RTL استفاده کنید (مانند وزیرمتن، Fira Code با Nerd Font)
- **Locale** — اطمینان حاصل کنید که Locale شما از UTF-8 پشتیبانی می‌کند (`LANG=en_US.UTF-8`)
- **جهت** — برخی ترمینال‌ها ممکن است نیاز به پیکربندی صریح RTL داشته باشند

### ادغام با شل

اسکریپت کمکی راست‌نگار برای zsh را به فایل پیکربندی شل خود اضافه کنید:

```bash
curl -fsSL https://raw.githubusercontent.com/RustNegar/dosu/main/zsh/rustnegar.zsh >> ~/.zshrc
```

یا یک میانبر سریع به `~/.bashrc` یا `~/.zshrc` اضافه کنید:

```bash
alias d='dosu'
```

<br>

## مشکلات شناخته‌شده

دوسو به‌طور گسترده آزمایش شده است، اما برخی ترکیب‌های ترمینال/شل ممکن است نیاز به پیکربندی دستی داشته باشند:

<table>
  <tr>
    <td width="65" align="center"><img src="assets/icons/issues.svg" width="28" alt="مشکل"/></td>
    <td><strong>ترمینال Kitty</strong><br/>ممکن است نیاز به تنظیم <code>force_ltr</code> در <code>kitty.conf</code> باشد.</td>
  </tr>
  <tr>
    <td width="65" align="center"><img src="assets/icons/issues.svg" width="28" alt="مشکل"/></td>
    <td><strong>tmux</strong><br/>تداخل پلاگین ممکن است رخ دهد؛ تنظیمات <code>tmux-navigator</code> را بررسی کنید.</td>
  </tr>
  <tr>
    <td width="65" align="center"><img src="assets/icons/issues.svg" width="28" alt="مشکل"/></td>
    <td><strong>ویجت FZF</strong><br/>رپرهای سفارشی ممکن است نیاز به تنظیم برای RTL داشته باشند.</td>
  </tr>
</table>

برای تشخیص خودکار و دریافت توصیه‌های تعمیر، دستور `dosu doctor` را اجرا کنید.

برای عیب‌یابی دقیق‌تر، به [Wiki](https://github.com/RustNegar/dosu/wiki/Troubleshooting-fa) مراجعه کنید.

<br>

## تماس

<table>
  <tr>
    <td align="center" width="65"><img src="assets/icons/contact.svg" width="36" alt="تماس"/></td>
    <td>
      <strong>مخزن</strong>: <a href="https://github.com/RustNegar/dosu">github.com/RustNegar/dosu</a><br/>
      <strong>هستهٔ اصلی</strong>: <a href="https://github.com/RustNegar/dosu-core">github.com/RustNegar/dosu-core</a><br/>
      <strong>نویسنده</strong>: کوروش میرحاجیان<br/>
      <strong>مجوز</strong>: MIT
    </td>
  </tr>
</table>

<br>

## حمایت از پروژه

اگر دوسو برایتان مفید بوده، حمایت از توسعهٔ مداومش را در نظر بگیرید. هر کمکی، کوچک یا بزرگ، بی‌نهایت ارزشمند است.

برای حمایت ریالی از داخل ایران، می‌توانید از کافیته استفاده کنید:

☕ [صفحه کافیته من](https://www.coffeete.ir/Kurosh_Mirhajian)

برای حمایت بین‌المللی، می‌توانید از آدرس‌های ارز دیجیتال زیر استفاده کنید:

</div>

<div dir="ltr" align="center">

| Network                                                                                                          | Address                                            |
| :--------------------------------------------------------------------------------------------------------------- | :------------------------------------------------- |
| ![TON](https://img.shields.io/badge/TON-0088CC?style=flat-square&logo=ton&logoColor=white)                       | `UQDPxrimgBU6Mil0dhDn0Fc303RLRXKr9hGGDu7bTEBdGGqs` |
| ![TRC20](<https://img.shields.io/badge/TRC20%20(Tron)-FF060A?style=flat-square&logo=tron&logoColor=white>)       | `TXix7uf6JPUKvWeUbA4A7wmQLVKnDbLRQU`               |
| ![ETH](<https://img.shields.io/badge/ERC20%20(Ethereum)-3C3C3D?style=flat-square&logo=ethereum&logoColor=white>) | `0x1FC907d3396460f1Cd94E3BC48564b1b46b70026`       |

</div>

<br>

<div align="center">
  <p>ساخته‌شده با ❤️ با استفاده از Rust</p>
  <p>
    <img src="https://img.shields.io/badge/Made%20with-Rust-orange?style=flat-square&logo=rust&logoColor=white" alt="ساخته‌شده با Rust">
  </p>
</div>
