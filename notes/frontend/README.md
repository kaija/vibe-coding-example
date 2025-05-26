# Notes App Frontend

A modern, responsive single-page application for managing personal notes, built with vanilla HTML, CSS, and JavaScript.

## Features

### Authentication
- **User Registration**: Create a new account with username and password
- **User Login**: Secure login with JWT token authentication
- **Auto-login**: Remembers user session using localStorage
- **Logout**: Secure logout with token cleanup

### Notes Management
- **Create Notes**: Add new notes with title and content
- **View Notes**: Display all notes in a responsive grid layout
- **Edit Notes**: Update existing notes inline
- **Delete Notes**: Remove notes with confirmation dialog
- **Search Notes**: Real-time search through note titles and content

### User Experience
- **Responsive Design**: Works seamlessly on desktop, tablet, and mobile devices
- **Loading States**: Visual feedback during API operations
- **Error Handling**: User-friendly error messages and validation
- **Keyboard Shortcuts**: 
  - `Ctrl/Cmd + N`: Create new note
  - `Escape`: Close modals
- **Accessibility**: Proper focus management and ARIA labels

## File Structure

```
frontend/
├── index.html          # Main HTML structure
├── styles.css          # Complete CSS styling with responsive design
├── app.js             # JavaScript application logic
└── README.md          # This documentation
```

## Getting Started

### Prerequisites
- The Rust backend API must be running on `http://localhost:8000`
- A modern web browser with JavaScript enabled

### Running the Frontend

1. **Serve the files**: You can use any static file server. Here are a few options:

   **Option 1: Python (if installed)**
   ```bash
   cd frontend
   python -m http.server 3000
   ```

   **Option 2: Node.js (if installed)**
   ```bash
   cd frontend
   npx serve -p 3000
   ```

   **Option 3: PHP (if installed)**
   ```bash
   cd frontend
   php -S localhost:3000
   ```

   **Option 4: Open directly in browser**
   - Simply open `index.html` in your web browser
   - Note: Some features may not work due to CORS restrictions

2. **Access the application**: Open your browser and navigate to `http://localhost:3000`

## API Integration

The frontend integrates with the following backend endpoints:

### Authentication Endpoints
- `POST /api/register` - User registration
- `POST /api/login` - User login
- `GET /api/me` - Get current user info

### Notes Endpoints
- `GET /api/notes` - Fetch all user notes
- `POST /api/notes` - Create a new note
- `PUT /api/notes/{id}` - Update an existing note
- `DELETE /api/notes/{id}` - Delete a note

## Configuration

### API Base URL
The API base URL is configured in `app.js`:
```javascript
const API_BASE_URL = 'http://localhost:8000';
```

To change the backend URL, modify this constant.

### Token Storage
JWT tokens are stored in `localStorage` for persistent sessions. The token is automatically included in all authenticated API requests.

## Usage Guide

### First Time Setup
1. Open the application in your browser
2. Click "Register here" to create a new account
3. Fill in your username (minimum 3 characters) and password (minimum 6 characters)
4. Click "Register" to create your account

### Managing Notes
1. **Create a Note**: Click the "Add New Note" button or use `Ctrl/Cmd + N`
2. **Edit a Note**: Hover over a note card and click the "Edit" button
3. **Delete a Note**: Hover over a note card and click the "Delete" button, then confirm
4. **Search Notes**: Use the search box to filter notes by title or content

### Navigation
- The application automatically saves your login session
- Use the "Logout" button in the header to sign out
- The search function works in real-time as you type

## Browser Compatibility

- **Modern Browsers**: Chrome 60+, Firefox 55+, Safari 12+, Edge 79+
- **Mobile Browsers**: iOS Safari 12+, Chrome Mobile 60+
- **Features Used**: ES6+ JavaScript, CSS Grid, Flexbox, Fetch API

## Security Features

- **JWT Authentication**: Secure token-based authentication
- **Input Validation**: Client-side and server-side validation
- **XSS Protection**: HTML escaping for user content
- **HTTPS Ready**: Works with HTTPS backends
- **Token Expiration**: Automatic logout on token expiration

## Responsive Design

The application is fully responsive with breakpoints at:
- **Desktop**: 1200px+ (full features)
- **Tablet**: 768px-1199px (adapted layout)
- **Mobile**: <768px (mobile-optimized interface)

## Accessibility Features

- **Keyboard Navigation**: Full keyboard support
- **Screen Reader Support**: Proper ARIA labels and semantic HTML
- **High Contrast Mode**: Supports system high contrast preferences
- **Reduced Motion**: Respects user's motion preferences
- **Focus Management**: Clear focus indicators and logical tab order

## Troubleshooting

### Common Issues

1. **"Failed to fetch" errors**
   - Ensure the backend API is running on `http://localhost:8000`
   - Check that CORS is properly configured in the backend

2. **Login not persisting**
   - Check if localStorage is enabled in your browser
   - Ensure you're not in private/incognito mode

3. **Notes not loading**
   - Verify your authentication token is valid
   - Check the browser console for error messages

4. **Styling issues**
   - Ensure `styles.css` is properly linked
   - Check for browser compatibility issues

### Development Tips

- Open browser Developer Tools (F12) to see console logs and network requests
- The application logs API errors to the console for debugging
- Use the Network tab to monitor API requests and responses

## Performance Considerations

- **Lazy Loading**: Notes are fetched only when needed
- **Efficient Rendering**: DOM updates are minimized
- **Caching**: User session is cached in localStorage
- **Optimized Assets**: CSS and JS are minified for production use

## Future Enhancements

Potential improvements for future versions:
- Rich text editor for note content
- Note categories and tags
- Export/import functionality
- Collaborative editing
- Offline support with service workers
- Dark mode toggle
- Note sharing capabilities