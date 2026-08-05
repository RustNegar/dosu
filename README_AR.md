<div dir="rtl" align="center">
  <img src="assets/logo.png" alt="شعار دوسو" width="150" height="150">
  <h1>دوسو (Dosu)</h1>
  <p>غلاف حديث ومتعدد المنصّات للطرفية (Terminal) يدعم النصوص ثنائية الاتجاه — مشروع من راست‌نگار (RustNegar)</p>
  <br>
  <p>
    <img src="https://img.shields.io/badge/rust-+v1.7-orange?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
    <img src="https://img.shields.io/badge/version-0.1.0-22C8E6?style=for-the-badge" alt="الإصدار">
    <img src="https://img.shields.io/badge/License-MIT-blue?style=for-the-badge" alt="الترخيص">
    <img src="https://img.shields.io/badge/Platform-Linux%20%7C%20macOS-lightgrey?style=for-the-badge" alt="المنصّة">
  </p>
  <br>
  <p>
    <a href="README.md">English</a> •
    <a href="README_FA.md">فارسی</a> •
    <b>العربية</b> •
    <a href="README_HE.md">עברית</a>
  </p>
  <br>
  <table>
    <tr>
      <td align="center" width="110"><a href="#نبذة"><img src="assets/icons/about.svg" width="44" alt="نبذة"/><br/><sub><b>نبذة</b></sub></a></td>
      <td align="center" width="110"><a href="#التثبيت"><img src="assets/icons/install.svg" width="44" alt="التثبيت"/><br/><sub><b>التثبيت</b></sub></a></td>
      <td align="center" width="110"><a href="#الاستخدام"><img src="assets/icons/usage.svg" width="44" alt="الاستخدام"/><br/><sub><b>الاستخدام</b></sub></a></td>
      <td align="center" width="110"><a href="#الإعدادات"><img src="assets/icons/config.svg" width="44" alt="الإعدادات"/><br/><sub><b>الإعدادات</b></sub></a></td>
      <td align="center" width="110"><a href="#المشاكل-المعروفة"><img src="assets/icons/issues.svg" width="44" alt="المشاكل"/><br/><sub><b>المشاكل</b></sub></a></td>
      <td align="center" width="110"><a href="#تواصل"><img src="assets/icons/contact.svg" width="44" alt="تواصل"/><br/><sub><b>تواصل</b></sub></a></td>
    </tr>
  </table>
</div>

<br>

<div dir="rtl">

## نبذة

**دوسو** غلاف متقدّم للطرفية (Terminal) مصمَّم خصيصًا لمعالجة وعرض النصوص ثنائية الاتجاه (Bidirectional). بُني هذا المشروع بلغة Rust فوق [`dosu-core`](https://github.com/RustNegar/dosu-core)، ويحلّ التحديات المعقّدة لعرض اللغات من اليمين إلى اليسار مثل العربية والفارسية داخل بيئة الطرفية، دون أن يُخلّ بالأدوات التي تستخدمها بالفعل.

<br>

## التثبيت

### التثبيت السريع (لينكس / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/RustNegar/dosu/main/install.sh | sh
```

### Homebrew (macOS)

```bash
brew install rustnegar/dosu/dosu
```

### البناء من المصدر

```bash
# استنساخ المستودع
git clone https://github.com/RustNegar/dosu.git
cd dosu

# البناء في وضع Release
cargo build --release

# تثبيت الملف التنفيذي
cargo install --path .
```

### المتطلبات

- Rust الإصدار ١.٧٠ أو أحدث (مطلوب فقط للبناء من المصدر)
- محاكي طرفية متوافق (Kitty، iTerm2، Ghostty، WezTerm وغيرها)
- نظام تشغيل شبيه بيونكس (لينكس أو macOS)

<br>

## الاستخدام

### الاستخدام الأساسي

لبدء جلسة طرفية ثنائية الاتجاه جديدة، ما عليك سوى تشغيل الأمر التالي:

```bash
dosu
```

### أمر التشخيص

شغّل أداة التشخيص المدمجة للتحقّق من بيئتك:

```bash
dosu doctor
```

يتحقّق هذا الأمر ممّا يلي:

- توافق الطرفية
- تعارضات الإعدادات المعروفة (Kitty، tmux، وضع vi)
- ضبط متغيّرات البيئة
- الحلول المقترحة

### وضع التصحيح

فعّل التسجيل التفصيلي أثناء التطوير:

```bash
export DOSU_DEBUG_DIR=/tmp/dosu-debug
dosu
```

ينشئ هذا ثلاثة ملفات سجلّ:

| الملف                  | المحتوى                               |
| ---------------------- | ------------------------------------- |
| `child_to_dosu.log`    | البايتات الخام من العملية الفرعية     |
| `dosu_to_child.log`    | البايتات المكتوبة إلى العملية الفرعية |
| `dosu_to_terminal.log` | مخرجات العرض إلى الطرفية              |

### خيارات سطر الأوامر

```bash
dosu --help
```

<br>

## الإعدادات

يعمل دوسو افتراضيًا بإعدادات منطقية جاهزة. ومع ذلك، قد تحتاج إلى تعديل إعدادات الطرفية أو الصَدَفة (shell) للحصول على أفضل تجربة.

### إعدادات الطرفية الموصى بها

- **الخط** — استخدم خطًا يدعم العربية بشكل جيّد (مثل Vazirmatn، أو Fira Code مع Nerd Font)
- **الترميز (Locale)** — تأكّد من أنّ الترميز لديك يدعم UTF-8 (`LANG=en_US.UTF-8`)
- **الاتجاه** — قد تحتاج بعض الطرفيات إلى ضبط صريح للاتجاه من اليمين إلى اليسار

### التكامل مع الصَدَفة

أضف سكربت راست‌نگار المساعد لـ zsh إلى ملف إعدادات الصَدَفة لديك:

```bash
curl -fsSL https://raw.githubusercontent.com/RustNegar/dosu/main/zsh/rustnegar.zsh >> ~/.zshrc
```

أو أضف اختصارًا سريعًا إلى `~/.bashrc` أو `~/.zshrc`:

```bash
alias d='dosu'
```

<br>

## المشاكل المعروفة

تمّ اختبار دوسو على نطاق واسع، لكن بعض تركيبات الطرفية/الصَدَفة قد تحتاج إلى إعداد يدوي:

<table>
  <tr>
    <td width="65" align="center"><img src="assets/icons/issues.svg" width="28" alt="مشكلة"/></td>
    <td><strong>طرفية Kitty</strong><br/>قد تحتاج إلى ضبط <code>force_ltr</code> في <code>kitty.conf</code>.</td>
  </tr>
  <tr>
    <td width="65" align="center"><img src="assets/icons/issues.svg" width="28" alt="مشكلة"/></td>
    <td><strong>tmux</strong><br/>قد تحدث تعارضات في الإضافات — تحقّق من إعدادات <code>tmux-navigator</code>.</td>
  </tr>
  <tr>
    <td width="65" align="center"><img src="assets/icons/issues.svg" width="28" alt="مشكلة"/></td>
    <td><strong>أداة FZF</strong><br/>قد تحتاج الأغلفة المخصّصة إلى تعديل لدعم الاتجاه من اليمين إلى اليسار.</td>
  </tr>
</table>

للكشف التلقائي والحصول على توصيات الإصلاح، شغّل الأمر `dosu doctor`.

لاستكشاف الأخطاء وإصلاحها بتفصيل أكبر، راجع [الويكي](https://github.com/RustNegar/dosu/wiki/Troubleshooting).

<br>

## تواصل

<table>
  <tr>
    <td align="center" width="65"><img src="assets/icons/contact.svg" width="36" alt="تواصل"/></td>
    <td>
      <strong>المستودع</strong>: <a href="https://github.com/RustNegar/dosu">github.com/RustNegar/dosu</a><br/>
      <strong>المحرّك الأساسي</strong>: <a href="https://github.com/RustNegar/dosu-core">github.com/RustNegar/dosu-core</a><br/>
      <strong>المؤلّف</strong>: كوروش ميرحاجيان<br/>
      <strong>الترخيص</strong>: MIT
    </td>
  </tr>
</table>

</div>

<br>

<div align="center">
  <p>صُنع بـ ❤️ باستخدام Rust</p>
  <p>
    <img src="https://img.shields.io/badge/Made%20with-Rust-orange?style=flat-square&logo=rust&logoColor=white" alt="صُنع باستخدام Rust">
  </p>
</div>
