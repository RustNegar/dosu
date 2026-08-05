<div dir="rtl" align="center">
  <img src="assets/logo.png" alt="לוגו Dosu" width="150" height="150">
  <h1>Dosu (דוסו)</h1>
  <p>עטיפת טרמינל מודרנית וחוצת-פלטפורמות עם תמיכה בטקסט דו-כיווני — פרויקט של RustNegar</p>
  <br>
  <p>
    <img src="https://img.shields.io/badge/rust-+v1.7-orange?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
    <img src="https://img.shields.io/badge/version-0.1.0-22C8E6?style=for-the-badge" alt="גרסה">
    <img src="https://img.shields.io/badge/License-MIT-blue?style=for-the-badge" alt="רישיון">
    <img src="https://img.shields.io/badge/Platform-Linux%20%7C%20macOS-lightgrey?style=for-the-badge" alt="פלטפורמה">
  </p>
  <br>
  <p>
    <a href="README.md">English</a> •
    <a href="README_FA.md">فارسی</a> •
    <a href="README_AR.md">العربية</a> •
    <b>עברית</b>
  </p>
  <br>
  <table>
    <tr>
      <td align="center" width="110"><a href="#אודות"><img src="assets/icons/about.svg" width="44" alt="אודות"/><br/><sub><b>אודות</b></sub></a></td>
      <td align="center" width="110"><a href="#התקנה"><img src="assets/icons/install.svg" width="44" alt="התקנה"/><br/><sub><b>התקנה</b></sub></a></td>
      <td align="center" width="110"><a href="#שימוש"><img src="assets/icons/usage.svg" width="44" alt="שימוש"/><br/><sub><b>שימוש</b></sub></a></td>
      <td align="center" width="110"><a href="#הגדרות"><img src="assets/icons/config.svg" width="44" alt="הגדרות"/><br/><sub><b>הגדרות</b></sub></a></td>
      <td align="center" width="110"><a href="#בעיות-ידועות"><img src="assets/icons/issues.svg" width="44" alt="בעיות"/><br/><sub><b>בעיות</b></sub></a></td>
      <td align="center" width="110"><a href="#יצירת-קשר"><img src="assets/icons/contact.svg" width="44" alt="יצירת קשר"/><br/><sub><b>יצירת קשר</b></sub></a></td>
    </tr>
  </table>
</div>

<br>

<div dir="rtl">

## אודות

**Dosu** היא עטיפת טרמינל מתקדמת שנועדה לטפל בטקסט דו-כיווני (Bidirectional) בצורה חלקה. הפרויקט נבנה ב-Rust על גבי [`dosu-core`](https://github.com/RustNegar/dosu-core), והוא פותר את האתגרים המורכבים של הצגת שפות הנכתבות מימין לשמאל, כמו עברית, פרסית וערבית, בתוך סביבת הטרמינל — בלי לשבש את הכלים שכבר בשימושכם.

<br>

## התקנה

### התקנה מהירה (Linux/macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/RustNegar/dosu/main/install.sh | sh
```

### Homebrew (macOS)

```bash
brew install rustnegar/dosu/dosu
```

### בנייה מהמקור

```bash
# שכפול המאגר
git clone https://github.com/RustNegar/dosu.git
cd dosu

# בנייה במצב Release
cargo build --release

# התקנת הקובץ הבינארי
cargo install --path .
```

### דרישות

- Rust גרסה 1.70 ומעלה (נדרש רק לבנייה מהמקור)
- אמולטור טרמינל תואם (Kitty, iTerm2, Ghostty, WezTerm וכו')
- מערכת הפעלה דמוית-יוניקס (Linux או macOS)

<br>

## שימוש

### שימוש בסיסי

כדי להתחיל הפעלת טרמינל דו-כיוונית חדשה, פשוט הריצו:

```bash
dosu
```

### פקודת אבחון

הריצו את כלי האבחון המובנה כדי לבדוק את הסביבה שלכם:

```bash
dosu doctor
```

הפקודה בודקת את הדברים הבאים:

- תאימות הטרמינל
- התנגשויות תצורה ידועות (Kitty, tmux, מצב vi)
- הגדרת משתני סביבה
- פתרונות מומלצים

### מצב ניפוי שגיאות

הפעילו רישום (logging) מפורט לצורכי פיתוח:

```bash
export DOSU_DEBUG_DIR=/tmp/dosu-debug
dosu
```

פעולה זו יוצרת שלושה קובצי לוג:

| קובץ                   | תוכן                           |
| ---------------------- | ------------------------------ |
| `child_to_dosu.log`    | בייטים גולמיים מהתהליך הבן     |
| `dosu_to_child.log`    | בייטים שנכתבו בחזרה לתהליך הבן |
| `dosu_to_terminal.log` | פלט הרינדור לטרמינל            |

### אפשרויות שורת פקודה

```bash
dosu --help
```

<br>

## הגדרות

Dosu עובד מהקופסה עם ברירות מחדל הגיוניות. עם זאת, ייתכן שתצטרכו להתאים את הגדרות הטרמינל או המעטפת (shell) לחוויה הטובה ביותר.

### הגדרות טרמינל מומלצות

- **גופן** — השתמשו בגופן עם תמיכה טובה ב-RTL (למשל Vazirmatn, או Fira Code עם Nerd Font)
- **Locale** — ודאו שה-locale שלכם תומך ב-UTF-8 (`LANG=en_US.UTF-8`)
- **כיווניות** — חלק מהטרמינלים עשויים לדרוש הגדרת RTL מפורשת

### שילוב עם המעטפת (Shell)

הוסיפו את סקריפט העזר של RustNegar עבור zsh לקובץ התצורה של המעטפת שלכם:

```bash
curl -fsSL https://raw.githubusercontent.com/RustNegar/dosu/main/zsh/rustnegar.zsh >> ~/.zshrc
```

או הוסיפו קיצור מהיר ל-`~/.bashrc` או `~/.zshrc`:

```bash
alias d='dosu'
```

<br>

## בעיות ידועות

Dosu נבדק באופן נרחב, אך שילובים מסוימים של טרמינל/מעטפת עשויים לדרוש תצורה ידנית:

<table>
  <tr>
    <td width="65" align="center"><img src="assets/icons/issues.svg" width="28" alt="בעיה"/></td>
    <td><strong>טרמינל Kitty</strong><br/>ייתכן שיידרש שינוי <code>force_ltr</code> בקובץ <code>kitty.conf</code>.</td>
  </tr>
  <tr>
    <td width="65" align="center"><img src="assets/icons/issues.svg" width="28" alt="בעיה"/></td>
    <td><strong>tmux</strong><br/>ייתכנו התנגשויות תוספים — בדקו את הגדרות <code>tmux-navigator</code>.</td>
  </tr>
  <tr>
    <td width="65" align="center"><img src="assets/icons/issues.svg" width="28" alt="בעיה"/></td>
    <td><strong>ווידג'ט FZF</strong><br/>עטיפות מותאמות אישית עשויות לדרוש התאמה עבור RTL.</td>
  </tr>
</table>

הריצו `dosu doctor` לאבחון אוטומטי והמלצות תיקון.

לפתרון בעיות מפורט יותר, ראו את ה[Wiki](https://github.com/RustNegar/dosu/wiki/Troubleshooting-he).

<br>

## יצירת קשר

<table>
  <tr>
    <td align="center" width="65"><img src="assets/icons/contact.svg" width="36" alt="יצירת קשר"/></td>
    <td>
      <strong>מאגר</strong>: <a href="https://github.com/RustNegar/dosu">github.com/RustNegar/dosu</a><br/>
      <strong>מנוע הליבה</strong>: <a href="https://github.com/RustNegar/dosu-core">github.com/RustNegar/dosu-core</a><br/>
      <strong>מחבר</strong>: Kurosh Mirhajian<br/>
      <strong>רישיון</strong>: MIT
    </td>
  </tr>
</table>

</div>

<br>

<div align="center">
  <p>נבנה באהבה ❤️ באמצעות Rust</p>
  <p>
    <img src="https://img.shields.io/badge/Made%20with-Rust-orange?style=flat-square&logo=rust&logoColor=white" alt="נבנה עם Rust">
  </p>
</div>
