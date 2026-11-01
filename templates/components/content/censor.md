{% component censor(orig=none, repl=none) -%}
{% if <utils.censored /> == "true" %}
    {%- if repl %} {{ repl | safe }} {% endif -%}
{% else %}
    {%- if body %}
        {{- body | safe -}}
    {% elif orig %}
        {{- orig | safe -}}
    {% endif -%}
{% endif %}
{%- endcomponent censor %}
