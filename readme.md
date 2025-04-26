# Rusc CSS ✨

**Rusc** is a lightweight, utility-first CSS framework built for speed and simplicity. 
 
Use Rusc to quickly build beautiful UIs without writing custom CSS!

---

## 🚀 Installation

### Option 1 — CDN (Recommended for Quick Start)

Add this line inside your `<head>` in your HTML file:

```html
<link rel="stylesheet" href="https://cdn.jsdelivr.net/gh/williampepple1/rusc/dist/output.css">
```

✅ Done! Now you can start using Rusc utility classes in your HTML.

---

## 🛠️ Usage

You can now use Rusc utility classes like:

```html
<h1 class="text-3xl text-red-500 font-bold underline">
  Hello World!
</h1>

<p class="mt-4 text-green-400">
  This paragraph is styled using Rusc CSS utilities.
</p>
```

✅ Supports:
- Background colors (`bg-red-100`, `bg-blue-500`, etc.)
- Text colors (`text-green-400`, etc.)
- Padding & Margin (`p-4`, `m-2`, etc.)
- Font size & font weight (`text-2xl`, `font-bold`, `font-light`, etc.)
- Hover, Focus, and Active states (`hover:bg-red-300`, `focus:text-blue-500`, `active:bg-green-400`)
- Dark mode utilities (`dark:bg-black`, `dark:text-white`)
- Responsive prefixes (`sm:`, `md:`, `lg:`, etc.)
- Background gradients (`bg-gradient-to-r`, `from-red-400`, `to-blue-500`, etc.)

---

## 🧩 React Usage

In your React project (e.g., in `index.html` or `public/index.html`), add the CDN link inside the `<head>`:

```html
<link rel="stylesheet" href="https://cdn.jsdelivr.net/gh/williampepple1/rusc/dist/output.css">
```

Or you can add it manually inside your `index.css` with:

```css
@import url('https://cdn.jsdelivr.net/gh/williampepple1/rusc/dist/output.css');
```

Then use it as normal:

```jsx
export default function App() {
  return (
    <div className="p-8 bg-blue-100 min-h-screen">
      <h1 className="text-4xl font-bold text-red-500">Hello Rusc!</h1>
      <p className="mt-4 text-green-500">Let's build something great.</p>
    </div>
  );
}
```

---

## 📦 NPM

install it using 

```bash
    npm i rusc
```

## 🎯 Contribution

Found a bug?  
Want to add more utilities?  
Feel free to fork the repo and submit a pull request!

---

## 📄 License

This project is licensed under the **MIT License** — free to use and modify.

---

# ✨ Links

- [GitHub Repository](https://github.com/williampepple1/rusc)
- [CDN via jsDelivr](https://cdn.jsdelivr.net/gh/williampepple1/rusc/dist/output.css)

---

# 🏆 Thank You for Using Rusc CSS!

---