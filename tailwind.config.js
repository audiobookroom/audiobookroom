/** @type {import('tailwindcss').Config} */
module.exports = {
    content: { 
      files: ["*.html","html_test/*.html", "./src/**/*.rs"],
    },
    theme: {
      extend: {},
    },
    plugins: [
      require('@tailwindcss/forms'),
    ],
  }