class TodoApp {
    constructor() {
        this.token = localStorage.getItem('token');
        this.user = JSON.parse(localStorage.getItem('user') || 'null');
        this.init();
    }

    init() {
        this.bindEvents();
        this.checkAuth();
    }

    bindEvents() {
        // Auth form events
        document.getElementById('login-form-element').addEventListener('submit', (e) => this.handleLogin(e));
        document.getElementById('register-form-element').addEventListener('submit', (e) => this.handleRegister(e));
        document.getElementById('logout-btn').addEventListener('click', () => this.handleLogout());

        // Todo form events
        document.getElementById('add-todo-form').addEventListener('submit', (e) => this.handleAddTodo(e));
    }

    checkAuth() {
        if (this.token && this.user) {
            this.showTodoSection();
            this.loadTodos();
        } else {
            this.showAuthSection();
        }
    }

    showAuthSection() {
        document.getElementById('auth-section').style.display = 'block';
        document.getElementById('todo-section').style.display = 'none';
    }

    showTodoSection() {
        document.getElementById('auth-section').style.display = 'none';
        document.getElementById('todo-section').style.display = 'block';
        document.getElementById('username-display').textContent = `Welcome, ${this.user.username}!`;
    }

    showLogin() {
        document.getElementById('login-tab').classList.add('active');
        document.getElementById('register-tab').classList.remove('active');
        document.getElementById('login-form').style.display = 'block';
        document.getElementById('register-form').style.display = 'none';
    }

    showRegister() {
        document.getElementById('register-tab').classList.add('active');
        document.getElementById('login-tab').classList.remove('active');
        document.getElementById('register-form').style.display = 'block';
        document.getElementById('login-form').style.display = 'none';
    }

    async handleLogin(e) {
        e.preventDefault();
        const formData = new FormData(e.target);
        const data = {
            username: formData.get('username'),
            password: formData.get('password')
        };

        try {
            const response = await fetch('/api/auth/login', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(data)
            });

            if (response.ok) {
                const result = await response.json();
                this.token = result.token;
                this.user = result.user;
                localStorage.setItem('token', this.token);
                localStorage.setItem('user', JSON.stringify(this.user));
                this.showMessage('Login successful!', 'success');
                this.showTodoSection();
                this.loadTodos();
                e.target.reset();
            } else {
                const error = await response.json();
                this.showMessage(error.error || 'Login failed', 'error');
            }
        } catch (error) {
            this.showMessage('Network error occurred', 'error');
        }
    }

    async handleRegister(e) {
        e.preventDefault();
        const formData = new FormData(e.target);
        const data = {
            username: formData.get('username'),
            email: formData.get('email'),
            password: formData.get('password')
        };

        try {
            const response = await fetch('/api/auth/register', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(data)
            });

            if (response.ok) {
                const result = await response.json();
                this.token = result.token;
                this.user = result.user;
                localStorage.setItem('token', this.token);
                localStorage.setItem('user', JSON.stringify(this.user));
                this.showMessage('Registration successful!', 'success');
                this.showTodoSection();
                this.loadTodos();
                e.target.reset();
            } else {
                const error = await response.json();
                this.showMessage(error.error || 'Registration failed', 'error');
            }
        } catch (error) {
            this.showMessage('Network error occurred', 'error');
        }
    }

    handleLogout() {
        this.token = null;
        this.user = null;
        localStorage.removeItem('token');
        localStorage.removeItem('user');
        this.showMessage('Logged out successfully', 'success');
        this.showAuthSection();
    }

    async handleAddTodo(e) {
        e.preventDefault();
        const formData = new FormData(e.target);
        
        // Get and validate scheduled_for
        const scheduledForValue = formData.get('scheduled_for');
        let scheduledFor = null;
        
        if (scheduledForValue) {
            const scheduledDate = new Date(scheduledForValue);
            const now = new Date();
            
            // Client-side validation: prevent scheduling in the past
            if (scheduledDate <= now) {
                this.showMessage('Cannot schedule todos in the past. Please select a future date and time.', 'error');
                return;
            }
            
            // Convert to ISO string for API
            scheduledFor = scheduledDate.toISOString();
        }
        
        const data = {
            title: formData.get('title'),
            description: formData.get('description') || null,
            scheduled_for: scheduledFor
        };

        try {
            const response = await fetch('/api/todos', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${this.token}`
                },
                body: JSON.stringify(data)
            });

            if (response.ok) {
                this.showMessage('Todo added successfully!', 'success');
                this.loadTodos();
                e.target.reset();
            } else {
                const error = await response.json();
                this.showMessage(error.error || 'Failed to add todo', 'error');
            }
        } catch (error) {
            this.showMessage('Network error occurred', 'error');
        }
    }

    async loadTodos() {
        try {
            const response = await fetch('/api/todos', {
                headers: {
                    'Authorization': `Bearer ${this.token}`
                }
            });

            if (response.ok) {
                const todos = await response.json();
                this.renderTodos(todos);
            } else {
                this.showMessage('Failed to load todos', 'error');
            }
        } catch (error) {
            this.showMessage('Network error occurred', 'error');
        }
    }

    renderTodos(todos) {
        const todoList = document.getElementById('todo-list');
        
        if (todos.length === 0) {
            todoList.innerHTML = `
                <div class="empty-state">
                    <h3>No todos yet</h3>
                    <p>Add your first todo above to get started!</p>
                </div>
            `;
            return;
        }

        todoList.innerHTML = todos.map(todo => `
            <div class="todo-item ${todo.completed ? 'completed' : ''} ${todo.scheduled_for ? 'scheduled' : 'unscheduled'}" data-id="${todo.id}">
                <div class="todo-header">
                    <div class="todo-title">
                        <input type="checkbox" class="todo-checkbox" ${todo.completed ? 'checked' : ''}
                               onchange="app.toggleTodo('${todo.id}', this.checked)">
                        <div class="todo-title-content">
                            <h4>${this.escapeHtml(todo.title)}</h4>
                            ${todo.scheduled_for ? `<div class="todo-schedule">📅 Scheduled for: ${this.formatScheduledDate(todo.scheduled_for)}</div>` : ''}
                        </div>
                    </div>
                    <div class="todo-actions">
                        <button class="btn btn-danger" onclick="app.deleteTodo('${todo.id}')">Delete</button>
                    </div>
                </div>
                ${todo.description ? `<div class="todo-description">${this.escapeHtml(todo.description)}</div>` : ''}
                <div class="todo-meta">
                    <div class="todo-dates">
                        <div>Created: ${this.formatDate(todo.created_at)}</div>
                        ${todo.updated_at !== todo.created_at ? `<div>Updated: ${this.formatDate(todo.updated_at)}</div>` : ''}
                    </div>
                    <div class="todo-status">
                        Status: <strong>${todo.completed ? 'Completed' : 'Pending'}</strong>
                        ${todo.scheduled_for ? `<span class="schedule-indicator">⏰</span>` : ''}
                    </div>
                </div>
            </div>
        `).join('');
    }

    async toggleTodo(todoId, completed) {
        try {
            // Get the current todo to preserve scheduled_for when updating
            const currentTodos = await this.getCurrentTodos();
            const currentTodo = currentTodos.find(t => t.id === todoId);
            
            const updateData = {
                completed,
                scheduled_for: currentTodo?.scheduled_for || null
            };

            const response = await fetch(`/api/todos/${todoId}`, {
                method: 'PUT',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${this.token}`
                },
                body: JSON.stringify(updateData)
            });

            if (response.ok) {
                this.loadTodos();
                this.showMessage(`Todo ${completed ? 'completed' : 'marked as pending'}!`, 'success');
            } else {
                const error = await response.json();
                this.showMessage(error.error || 'Failed to update todo', 'error');
                this.loadTodos(); // Reload to reset checkbox state
            }
        } catch (error) {
            this.showMessage('Network error occurred', 'error');
            this.loadTodos(); // Reload to reset checkbox state
        }
    }

    async getCurrentTodos() {
        try {
            const response = await fetch('/api/todos', {
                headers: {
                    'Authorization': `Bearer ${this.token}`
                }
            });
            if (response.ok) {
                return await response.json();
            }
            return [];
        } catch (error) {
            return [];
        }
    }

    async deleteTodo(todoId) {
        if (!confirm('Are you sure you want to delete this todo?')) {
            return;
        }

        try {
            const response = await fetch(`/api/todos/${todoId}`, {
                method: 'DELETE',
                headers: {
                    'Authorization': `Bearer ${this.token}`
                }
            });

            if (response.ok) {
                this.showMessage('Todo deleted successfully!', 'success');
                this.loadTodos();
            } else {
                const error = await response.json();
                this.showMessage(error.error || 'Failed to delete todo', 'error');
            }
        } catch (error) {
            this.showMessage('Network error occurred', 'error');
        }
    }

    showMessage(message, type) {
        const messageEl = document.getElementById('message');
        messageEl.textContent = message;
        messageEl.className = `message ${type}`;
        messageEl.style.display = 'block';

        setTimeout(() => {
            messageEl.style.display = 'none';
        }, 5000);
    }

    formatDate(dateString) {
        const date = new Date(dateString);
        return date.toLocaleString();
    }

    formatScheduledDate(dateString) {
        const date = new Date(dateString);
        const now = new Date();
        
        // Check if the scheduled date is in the past
        const isPast = date < now;
        
        // Format options for a more readable display
        const options = {
            year: 'numeric',
            month: 'short',
            day: 'numeric',
            hour: 'numeric',
            minute: '2-digit',
            hour12: true
        };
        
        const formattedDate = date.toLocaleDateString('en-US', options);
        
        // Add visual indicator for overdue items
        if (isPast) {
            return `<span class="overdue">${formattedDate} (Overdue)</span>`;
        }
        
        return formattedDate;
    }

    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }
}

// Global functions for HTML onclick handlers
function showLogin() {
    app.showLogin();
}

function showRegister() {
    app.showRegister();
}

// Initialize the app
const app = new TodoApp();