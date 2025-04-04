# Starforge
A research project into Wayland compositors

# Components
 - starforge-comp: The official reference implementation of the Starforge compositor. Users can build their own using the Starforge libraries to meet their specific needs, or use the official plugins library to extend the official version.
 - starforge-core: The core library for the Starforge compositor. This library provides the core Wayland protocol implementation
 - starforge-config: The configuration system for the Starforge compositor. This library provides a modular, extensible configuration system. Core configuration necessary to all compositors is included in structs, with the option to extend the configuration with plugin configs.
 - starforge-render: The extensible render pipeline for the Starforge compositor. This library provides a modular, extensible render pipeline. Core rendering functionality is included in structs, with the option to extend the rendering with the plugin library.
 - starforge-plugins: The official plugin library for writing Starforge plugins. This library provides an interface for extending a Starforge compositor with custom rendering effects, input handling, and other functionality. This library is used by the official Starforge compositor, but can also be used to build custom compositors using the Starforge libraries.
 - starforge-shell: COSMIC desktop environment integration, including libcosmic window decorations, workspace management, and tiling/floating window management.