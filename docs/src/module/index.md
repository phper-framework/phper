# Module

> Just to clarify, in PHP's source code, PHP extensions are named as "PHP modules."
>
> PHP extensions (aka PHP “modules”) are loaded in INI files with an “extension=pib.so” line.
>
> Refer: <https://www.phpinternalsbook.com/php7/extensions_design/zend_extensions.html#on-differences-between-php-and-zend-extensions>

[Module](https://github.com/phper-framework/phper/blob/master/phper/src/modules.rs) is the entry of registering hooks, functions,
classes, ini properties, constants, etc.
