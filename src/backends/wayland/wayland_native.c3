module wayland_native;
import std::io;

// ========== External C Bindings ==========
extern fn void* wl_display_connect(char* name);
extern fn int wl_display_dispatch(void* display);
extern fn void* wl_display_get_registry(void* display);
extern fn int wl_display_roundtrip(void* display);
extern fn void wl_registry_add_listener(void* registry, void* listener, void* data);
extern fn void* wl_registry_bind(void* registry, uint name, void* iface, uint version);
extern fn void* wl_compositor_create_surface(void* compositor);
extern fn void wl_surface_commit(void* surface);
extern fn void wl_surface_attach(void* surface, void* buffer, int x, int y);
extern fn void wl_surface_damage_buffer(void* surface, int x, int y, int w, int h);
extern fn void* wl_shm_create_pool(void* shm, int fd, int size);
extern fn void* wl_shm_pool_create_buffer(void* pool, int offset, int width, int height, int stride, uint format);
extern fn void wl_shm_pool_destroy(void* pool);
extern fn void wl_buffer_add_listener(void* buffer, void* listener, void* data);
extern fn void wl_buffer_destroy(void* buffer);
extern fn void* xdg_wm_base_get_xdg_surface(void* wm_base, void* surface);
extern fn void xdg_surface_add_listener(void* xdg_surface, void* listener, void* data);
extern fn void* xdg_surface_get_toplevel(void* xdg_surface);
extern fn void xdg_toplevel_add_listener(void* toplevel, void* listener, void* data);
extern fn void xdg_toplevel_set_title(void* toplevel, char* title);
extern fn void xdg_surface_ack_configure(void* xdg_surface, uint serial);
extern fn void xdg_wm_base_pong(void* wm_base, uint serial);
extern fn int shm_open(char* name, int oflag, int mode);
extern fn int shm_unlink(char* name);
extern fn int ftruncate(int fd, long length);
extern fn int close(int fd);
extern fn void* mmap(void* addr, long length, int prot, int flags, int fd, long offset);
extern fn int strcmp(char* s1, char* s2);

extern void* wl_compositor_interface;
extern void* wl_shm_interface;
extern void* xdg_wm_base_interface;

// Constants
const int O_RDWR                  = 2;
const int O_CREAT                 = 64;
const int O_EXCL                  = 128;
const int PROT_READ               = 1;
const int PROT_WRITE              = 2;
const int MAP_SHARED              = 1;
const uint WL_SHM_FORMAT_ARGB8888 = 0;

// ========== State Structure ==========
struct ClientState
{
	void* display;
	void* registry;
	void* compositor;
	void* shm;
	void* wm_base;
	void* surface;
	void* xdg_surface;
	void* xdg_toplevel;
	void* buffer;
	bool configured;
}

// ========== Listener Function Types ==========
alias PingFunc = fn void(void* data, void* wm_base, uint serial);
alias XdgSurfaceConfigureFunc = fn void(void* data, void* xdg_surface, uint serial);
alias XdgToplevelConfigureFunc = fn void(void* data, void* toplevel, int width, int height, void* states);
alias XdgToplevelCloseFunc = fn void(void* data, void* toplevel);
alias RegistryGlobalFunc = fn void(void* data, void* registry, uint name, char* iface, uint version);
alias RegistryGlobalRemoveFunc = fn void(void* data, void* registry, uint name);
alias BufferReleaseFunc = fn void(void* data, void* wl_buffer);

// Listener structs
struct XdgWmBaseListener
{
	PingFunc ping;
}

struct XdgSurfaceListener
{
	XdgSurfaceConfigureFunc configure;
}

struct XdgToplevelListener
{
	XdgToplevelConfigureFunc configure;
	XdgToplevelCloseFunc close;
}

struct WlRegistryListener
{
	RegistryGlobalFunc global;
	RegistryGlobalRemoveFunc global_remove;
}

struct WlBufferListener
{
	BufferReleaseFunc release;
}

// ========== Listener Implementations ==========
fn void wm_base_ping(void* data, void* wm_base, uint serial)
{
	xdg_wm_base_pong(wm_base, serial);
}

fn void xdg_surface_configure(void* data, void* xdg_surface, uint serial)
{
	ClientState* state = (ClientState*)data;
	xdg_surface_ack_configure(xdg_surface, serial);
	state.configured = true;
}

fn void xdg_toplevel_configure(void* data, void* toplevel, int width, int height, void* states)
{
	// Ignore requested size
}

fn void xdg_toplevel_close(void* data, void* toplevel)
{
	io::printn("Window closed.");
}

fn void registry_global(void* data, void* registry, uint name, char* iface, uint version)
{
	ClientState* state = (ClientState*)data;
	if (strcmp(iface, "wl_compositor") == 0)
	{
		state.compositor = wl_registry_bind(registry, name, &wl_compositor_interface, 4);
	}
	else if (strcmp(iface, "wl_shm") == 0)
	{
		state.shm = wl_registry_bind(registry, name, &wl_shm_interface, 1);
	}
	else if (strcmp(iface, "xdg_wm_base") == 0)
	{
		state.wm_base = wl_registry_bind(registry, name, &xdg_wm_base_interface, 1);
	}
}

fn void registry_global_remove(void* data, void* registry, uint name)
{
	// Empty
}

fn void buffer_release(void* data, void* wl_buffer)
{
	wl_buffer_destroy(wl_buffer);
}

fn int create_shm_file(usz size)
{
	char[32] name;
	name[0] = '/';
	name[1] = 'w';
	name[2] = 'l';
	name[3] = '_';
	name[4] = 's';
	name[5] = 'h';
	name[6] = 'm';
	name[7] = '-';
	int fd = -1;
	for (int i = 0; i < 100; i++)
	{
		name[8] = (char)('A' + (i % 26));
		name[9] = 0;
		fd = shm_open(&name, O_RDWR | O_CREAT | O_EXCL, 0600);
		if (fd >= 0)
		{
			shm_unlink(&name);
			break;
		}
	}
	if (fd < 0) return -1;
	if (ftruncate(fd, (long)size) < 0)
	{
		close(fd);
		return -1;
	}
	return fd;
}

fn bool wayland_init() 
@cname("wayland_init")
@export
{
	ClientState state = { };

	state.display = wl_display_connect(null);
	if (state.display == null)
	{
		io::eprintn("Failed to connect");
		return false;
	}

	state.registry = wl_display_get_registry(state.display);

	WlRegistryListener reg_listener = {
		.global = &registry_global,
		.global_remove = &registry_global_remove,
	};
	wl_registry_add_listener(state.registry, &reg_listener, &state);
	wl_display_roundtrip(state.display);

	if (state.compositor == null || state.shm == null || state.wm_base == null)
	{
		io::eprintn("Missing required globals");
		return false;
	}

	state.surface = wl_compositor_create_surface(state.compositor);

	state.xdg_surface = xdg_wm_base_get_xdg_surface(state.wm_base, state.surface);
	XdgSurfaceListener surf_listener = { .configure = &xdg_surface_configure };
	xdg_surface_add_listener(state.xdg_surface, &surf_listener, &state);

	state.xdg_toplevel = xdg_surface_get_toplevel(state.xdg_surface);
	XdgToplevelListener top_listener = {
		.configure = &xdg_toplevel_configure,
		.close = &xdg_toplevel_close,
	};
	xdg_toplevel_add_listener(state.xdg_toplevel, &top_listener, &state);
	xdg_toplevel_set_title(state.xdg_toplevel, "Simple Wayland Window");

	wl_surface_commit(state.surface);

	while (!state.configured && wl_display_dispatch(state.display) != -1)
	{
		// Wait for configure
	}

	int width  = 640;
	int height = 480;
	int stride = width * 4;
	int size   = stride * height;
	int fd     = create_shm_file((usz)size);
	if (fd < 0)
	{
		io::eprintn("Failed to create shm file");
		return false;
	}

	uint* data = (uint*)mmap(null, (long)size, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
	for (int i = 0; i < width * height; i++)
	{
		data[i] = 0xFF336699;
	}

	void* pool = wl_shm_create_pool(state.shm, fd, size);
	state.buffer = wl_shm_pool_create_buffer(pool, 0, width, height, stride, WL_SHM_FORMAT_ARGB8888);
	WlBufferListener buf_listener = { .release = &buffer_release };
	wl_buffer_add_listener(state.buffer, &buf_listener, &state);
	wl_shm_pool_destroy(pool);
	close(fd);

	wl_surface_attach(state.surface, state.buffer, 0, 0);
	wl_surface_damage_buffer(state.surface, 0, 0, width, height);
	wl_surface_commit(state.surface);

	while (wl_display_dispatch(state.display) != -1)
	{
		// Event loop
	}
	return true;
}
