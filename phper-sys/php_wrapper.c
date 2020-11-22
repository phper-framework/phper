#include "php_wrapper.h"

zend_string *phper_zend_string_init(const char *str, size_t len, int persistent) {
    return zend_string_init(str, len, persistent);
}

zend_string *phper_zend_new_interned_string(zend_string *str) {
    return zend_new_interned_string(str);
}

zend_class_entry phper_init_class_entry(const char *class_name, const zend_function_entry *functions) {
    zend_class_entry class_container;
    INIT_CLASS_ENTRY(class_container, class_name, functions);
    return class_container;
}

void phper_zval_string(zval *return_value, const char *s) {
    ZVAL_STRING(return_value, s);
}

zend_uchar phper_zval_get_type(const zval* pz) {
    return zval_get_type(pz);
}

void phper_zval_stringl(zval *return_value, const char *s, size_t len) {
    ZVAL_STRINGL(return_value, s, len);
}

char *phper_z_strval_p(const zval *v) {
    return Z_STRVAL_P(v);
}

zval *phper_get_this(zend_execute_data *execute_data) {
    return getThis();
}
