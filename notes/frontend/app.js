// API Configuration
const API_BASE_URL = 'http://localhost:8000';
const API_ENDPOINTS = {
    register: '/api/register',
    login: '/api/login',
    me: '/api/me',
    notes: '/api/notes'
};

// Application State
let currentUser = null;
let notes = [];
let currentEditingNote = null;
let currentDeletingNote = null;

// DOM Elements
const elements = {
    // Sections
    authSection: document.getElementById('auth-section'),
    appSection: document.getElementById('app-section'),
    loadingOverlay: document.getElementById('loading-overlay'),
    
    // Auth forms
    loginForm: document.getElementById('login-form'),
    registerForm: document.getElementById('register-form'),
    loginFormElement: document.getElementById('login-form-element'),
    registerFormElement: document.getElementById('register-form-element'),
    
    // Auth form switches
    showRegisterLink: document.getElementById('show-register'),
    showLoginLink: document.getElementById('show-login'),
    
    // User info
    usernameDisplay: document.getElementById('username-display'),
    logoutBtn: document.getElementById('logout-btn'),
    
    // Notes
    notesGrid: document.getElementById('notes-grid'),
    noNotesSection: document.getElementById('no-notes'),
    addNoteBtn: document.getElementById('add-note-btn'),
    searchInput: document.getElementById('search-input'),
    
    // Modals
    noteModal: document.getElementById('note-modal'),
    deleteModal: document.getElementById('delete-modal'),
    noteForm: document.getElementById('note-form'),
    modalTitle: document.getElementById('modal-title'),
    noteTitle: document.getElementById('note-title'),
    noteContent: document.getElementById('note-content'),
    
    // Modal buttons
    closeModal: document.getElementById('close-modal'),
    cancelNote: document.getElementById('cancel-note'),
    saveNote: document.getElementById('save-note'),
    closeDeleteModal: document.getElementById('close-delete-modal'),
    cancelDelete: document.getElementById('cancel-delete'),
    confirmDelete: document.getElementById('confirm-delete'),
    
    // Delete modal content
    deleteNoteTitle: document.getElementById('delete-note-title'),
    deleteNoteContent: document.getElementById('delete-note-content')
};

// Utility Functions
function showLoading() {
    elements.loadingOverlay.classList.remove('hidden');
}

function hideLoading() {
    elements.loadingOverlay.classList.add('hidden');
}

function showError(elementId, message) {
    const errorElement = document.getElementById(elementId);
    if (errorElement) {
        errorElement.textContent = message;
        errorElement.style.display = 'block';
    }
}

function clearError(elementId) {
    const errorElement = document.getElementById(elementId);
    if (errorElement) {
        errorElement.textContent = '';
        errorElement.style.display = 'none';
    }
}

function clearAllErrors() {
    const errorElements = document.querySelectorAll('.error-message');
    errorElements.forEach(element => {
        element.textContent = '';
        element.style.display = 'none';
    });
    
    const inputElements = document.querySelectorAll('input, textarea');
    inputElements.forEach(element => {
        element.classList.remove('error');
    });
}

function validateEmail(email) {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return emailRegex.test(email);
}

function formatDate(dateString) {
    const date = new Date(dateString);
    const now = new Date();
    const diffTime = Math.abs(now - date);
    const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));
    
    if (diffDays === 1) {
        return 'Today';
    } else if (diffDays === 2) {
        return 'Yesterday';
    } else if (diffDays <= 7) {
        return `${diffDays - 1} days ago`;
    } else {
        return date.toLocaleDateString();
    }
}

// Token Management
function getToken() {
    return localStorage.getItem('auth_token');
}

function setToken(token) {
    localStorage.setItem('auth_token', token);
}

function removeToken() {
    localStorage.removeItem('auth_token');
}

// API Functions
async function apiRequest(endpoint, options = {}) {
    const url = `${API_BASE_URL}${endpoint}`;
    const token = getToken();
    
    const defaultOptions = {
        headers: {
            'Content-Type': 'application/json',
            ...(token && { 'Authorization': `Bearer ${token}` })
        }
    };
    
    const finalOptions = {
        ...defaultOptions,
        ...options,
        headers: {
            ...defaultOptions.headers,
            ...options.headers
        }
    };
    
    try {
        const response = await fetch(url, finalOptions);
        
        if (!response.ok) {
            const errorData = await response.json().catch(() => ({}));
            throw new Error(errorData.message || `HTTP error! status: ${response.status}`);
        }
        
        // Handle 204 No Content responses
        if (response.status === 204) {
            return null;
        }
        
        return await response.json();
    } catch (error) {
        console.error('API request failed:', error);
        throw error;
    }
}

// Authentication Functions
async function register(username, password) {
    const response = await apiRequest(API_ENDPOINTS.register, {
        method: 'POST',
        body: JSON.stringify({ username, password })
    });
    
    setToken(response.token);
    currentUser = response.user;
    return response;
}

async function login(username, password) {
    const response = await apiRequest(API_ENDPOINTS.login, {
        method: 'POST',
        body: JSON.stringify({ username, password })
    });
    
    setToken(response.token);
    currentUser = response.user;
    return response;
}

async function getCurrentUser() {
    const response = await apiRequest(API_ENDPOINTS.me);
    currentUser = response;
    return response;
}

function logout() {
    removeToken();
    currentUser = null;
    notes = [];
    showAuthSection();
}

// Notes Functions
async function fetchNotes() {
    const response = await apiRequest(API_ENDPOINTS.notes);
    notes = response.notes || [];
    return notes;
}

async function createNote(title, content) {
    const response = await apiRequest(API_ENDPOINTS.notes, {
        method: 'POST',
        body: JSON.stringify({ title, content })
    });
    return response;
}

async function updateNote(noteId, title, content) {
    const response = await apiRequest(`${API_ENDPOINTS.notes}/${noteId}`, {
        method: 'PUT',
        body: JSON.stringify({ title, content })
    });
    return response;
}

async function deleteNote(noteId) {
    await apiRequest(`${API_ENDPOINTS.notes}/${noteId}`, {
        method: 'DELETE'
    });
}

// UI Functions
function showAuthSection() {
    elements.authSection.classList.remove('hidden');
    elements.appSection.classList.add('hidden');
    clearAllErrors();
}

function showAppSection() {
    elements.authSection.classList.add('hidden');
    elements.appSection.classList.remove('hidden');
    elements.usernameDisplay.textContent = currentUser.username;
}

function showLoginForm() {
    elements.loginForm.classList.remove('hidden');
    elements.registerForm.classList.add('hidden');
    clearAllErrors();
}

function showRegisterForm() {
    elements.loginForm.classList.add('hidden');
    elements.registerForm.classList.remove('hidden');
    clearAllErrors();
}

function showAddNoteModal() {
    currentEditingNote = null;
    elements.modalTitle.textContent = 'Add New Note';
    elements.noteTitle.value = '';
    elements.noteContent.value = '';
    elements.saveNote.textContent = 'Save Note';
    elements.noteModal.classList.remove('hidden');
    elements.noteTitle.focus();
    clearAllErrors();
}

function showEditNoteModal(note) {
    currentEditingNote = note;
    elements.modalTitle.textContent = 'Edit Note';
    elements.noteTitle.value = note.title;
    elements.noteContent.value = note.content;
    elements.saveNote.textContent = 'Update Note';
    elements.noteModal.classList.remove('hidden');
    elements.noteTitle.focus();
    clearAllErrors();
}

function hideNoteModal() {
    elements.noteModal.classList.add('hidden');
    currentEditingNote = null;
    clearAllErrors();
}

function showDeleteModal(note) {
    currentDeletingNote = note;
    elements.deleteNoteTitle.textContent = note.title;
    elements.deleteNoteContent.textContent = note.content;
    elements.deleteModal.classList.remove('hidden');
}

function hideDeleteModal() {
    elements.deleteModal.classList.add('hidden');
    currentDeletingNote = null;
}

function renderNotes(notesToRender = notes) {
    if (notesToRender.length === 0) {
        elements.notesGrid.innerHTML = '';
        elements.noNotesSection.classList.remove('hidden');
        return;
    }
    
    elements.noNotesSection.classList.add('hidden');
    
    elements.notesGrid.innerHTML = notesToRender.map(note => `
        <div class="note-card" data-note-id="${note.id}">
            <div class="note-header">
                <h3 class="note-title">${escapeHtml(note.title)}</h3>
                <div class="note-actions">
                    <button class="note-action-btn edit-btn" onclick="editNote('${note.id}')">
                        Edit
                    </button>
                    <button class="note-action-btn delete-btn" onclick="confirmDeleteNote('${note.id}')">
                        Delete
                    </button>
                </div>
            </div>
            <div class="note-content">${escapeHtml(note.content)}</div>
            <div class="note-meta">
                <span class="note-date">Created: ${formatDate(note.created_at)}</span>
                ${note.updated_at !== note.created_at ? 
                    `<span class="note-date">Updated: ${formatDate(note.updated_at)}</span>` : 
                    ''
                }
            </div>
        </div>
    `).join('');
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

function filterNotes(searchTerm) {
    if (!searchTerm.trim()) {
        renderNotes(notes);
        return;
    }
    
    const filteredNotes = notes.filter(note => 
        note.title.toLowerCase().includes(searchTerm.toLowerCase()) ||
        note.content.toLowerCase().includes(searchTerm.toLowerCase())
    );
    
    renderNotes(filteredNotes);
}

// Global functions for onclick handlers
window.editNote = function(noteId) {
    const note = notes.find(n => n.id === noteId);
    if (note) {
        showEditNoteModal(note);
    }
};

window.confirmDeleteNote = function(noteId) {
    const note = notes.find(n => n.id === noteId);
    if (note) {
        showDeleteModal(note);
    }
};

window.showAddNoteModal = showAddNoteModal;

// Form Validation
function validateLoginForm(formData) {
    const errors = {};
    
    if (!formData.username || formData.username.length < 3) {
        errors.username = 'Username must be at least 3 characters long';
    }
    
    if (!formData.password || formData.password.length < 6) {
        errors.password = 'Password must be at least 6 characters long';
    }
    
    return errors;
}

function validateRegisterForm(formData) {
    const errors = {};
    
    if (!formData.username || formData.username.length < 3) {
        errors.username = 'Username must be at least 3 characters long';
    }
    
    if (!formData.password || formData.password.length < 6) {
        errors.password = 'Password must be at least 6 characters long';
    }
    
    if (formData.password !== formData.confirmPassword) {
        errors.confirmPassword = 'Passwords do not match';
    }
    
    return errors;
}

function validateNoteForm(formData) {
    const errors = {};
    
    if (!formData.title || formData.title.trim().length === 0) {
        errors.title = 'Title is required';
    } else if (formData.title.length > 200) {
        errors.title = 'Title must be less than 200 characters';
    }
    
    if (!formData.content || formData.content.trim().length === 0) {
        errors.content = 'Content is required';
    } else if (formData.content.length > 5000) {
        errors.content = 'Content must be less than 5000 characters';
    }
    
    return errors;
}

function displayFormErrors(errors, prefix = '') {
    Object.keys(errors).forEach(field => {
        const errorId = `${prefix}${field}-error`;
        const inputId = `${prefix}${field}`;
        
        showError(errorId, errors[field]);
        
        const inputElement = document.getElementById(inputId);
        if (inputElement) {
            inputElement.classList.add('error');
        }
    });
}

// Event Listeners
function setupEventListeners() {
    // Auth form switches
    elements.showRegisterLink.addEventListener('click', (e) => {
        e.preventDefault();
        showRegisterForm();
    });
    
    elements.showLoginLink.addEventListener('click', (e) => {
        e.preventDefault();
        showLoginForm();
    });
    
    // Login form
    elements.loginFormElement.addEventListener('submit', async (e) => {
        e.preventDefault();
        clearAllErrors();
        
        const formData = new FormData(e.target);
        const data = {
            username: formData.get('username'),
            password: formData.get('password')
        };
        
        const errors = validateLoginForm(data);
        if (Object.keys(errors).length > 0) {
            displayFormErrors(errors, 'login-');
            return;
        }
        
        try {
            showLoading();
            await login(data.username, data.password);
            await loadNotesAndShowApp();
        } catch (error) {
            showError('login-error', error.message);
        } finally {
            hideLoading();
        }
    });
    
    // Register form
    elements.registerFormElement.addEventListener('submit', async (e) => {
        e.preventDefault();
        clearAllErrors();
        
        const formData = new FormData(e.target);
        const data = {
            username: formData.get('username'),
            password: formData.get('password'),
            confirmPassword: formData.get('confirmPassword')
        };
        
        const errors = validateRegisterForm(data);
        if (Object.keys(errors).length > 0) {
            displayFormErrors(errors, 'register-');
            return;
        }
        
        try {
            showLoading();
            await register(data.username, data.password);
            await loadNotesAndShowApp();
        } catch (error) {
            showError('register-error', error.message);
        } finally {
            hideLoading();
        }
    });
    
    // Logout
    elements.logoutBtn.addEventListener('click', logout);
    
    // Add note button
    elements.addNoteBtn.addEventListener('click', showAddNoteModal);
    
    // Search input
    elements.searchInput.addEventListener('input', (e) => {
        filterNotes(e.target.value);
    });
    
    // Note form
    elements.noteForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        clearAllErrors();
        
        const formData = new FormData(e.target);
        const data = {
            title: formData.get('title').trim(),
            content: formData.get('content').trim()
        };
        
        const errors = validateNoteForm(data);
        if (Object.keys(errors).length > 0) {
            displayFormErrors(errors, 'note-');
            return;
        }
        
        try {
            showLoading();
            
            if (currentEditingNote) {
                await updateNote(currentEditingNote.id, data.title, data.content);
            } else {
                await createNote(data.title, data.content);
            }
            
            await fetchNotes();
            renderNotes();
            hideNoteModal();
        } catch (error) {
            showError('note-form-error', error.message);
        } finally {
            hideLoading();
        }
    });
    
    // Modal close buttons
    elements.closeModal.addEventListener('click', hideNoteModal);
    elements.cancelNote.addEventListener('click', hideNoteModal);
    elements.closeDeleteModal.addEventListener('click', hideDeleteModal);
    elements.cancelDelete.addEventListener('click', hideDeleteModal);
    
    // Delete confirmation
    elements.confirmDelete.addEventListener('click', async () => {
        if (!currentDeletingNote) return;
        
        try {
            showLoading();
            await deleteNote(currentDeletingNote.id);
            await fetchNotes();
            renderNotes();
            hideDeleteModal();
        } catch (error) {
            console.error('Error deleting note:', error);
            alert('Failed to delete note. Please try again.');
        } finally {
            hideLoading();
        }
    });
    
    // Modal backdrop clicks
    elements.noteModal.addEventListener('click', (e) => {
        if (e.target === elements.noteModal) {
            hideNoteModal();
        }
    });
    
    elements.deleteModal.addEventListener('click', (e) => {
        if (e.target === elements.deleteModal) {
            hideDeleteModal();
        }
    });
    
    // Keyboard shortcuts
    document.addEventListener('keydown', (e) => {
        // Escape key closes modals
        if (e.key === 'Escape') {
            if (!elements.noteModal.classList.contains('hidden')) {
                hideNoteModal();
            } else if (!elements.deleteModal.classList.contains('hidden')) {
                hideDeleteModal();
            }
        }
        
        // Ctrl/Cmd + N opens new note modal
        if ((e.ctrlKey || e.metaKey) && e.key === 'n' && elements.appSection && !elements.appSection.classList.contains('hidden')) {
            e.preventDefault();
            showAddNoteModal();
        }
    });
    
    // Clear errors on input
    document.addEventListener('input', (e) => {
        if (e.target.matches('input, textarea')) {
            e.target.classList.remove('error');
            const errorId = e.target.id + '-error';
            clearError(errorId);
        }
    });
}

// App Initialization
async function loadNotesAndShowApp() {
    try {
        await fetchNotes();
        renderNotes();
        showAppSection();
    } catch (error) {
        console.error('Error loading notes:', error);
        logout();
    }
}

async function initializeApp() {
    setupEventListeners();
    
    const token = getToken();
    if (token) {
        try {
            showLoading();
            await getCurrentUser();
            await loadNotesAndShowApp();
        } catch (error) {
            console.error('Error initializing app:', error);
            removeToken();
            showAuthSection();
        } finally {
            hideLoading();
        }
    } else {
        showAuthSection();
    }
}

// Start the application
document.addEventListener('DOMContentLoaded', initializeApp);