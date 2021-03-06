# frozen_string_literal: true

module Inkoc
  module TypeSystem
    # A dynamic type that responds to every message.
    class Dynamic
      include Type
      include TypeWithPrototype
      include TypeWithAttributes
      include GenericType
      include NewInstance

      SINGLETON = new.freeze

      def self.singleton
        SINGLETON
      end

      def new_instance(*)
        self
      end

      def generic_type?
        false
      end

      def prototype
        nil
      end

      def prototype=(*)
        nil
      end

      def dynamic?
        true
      end

      def attributes
        SymbolTable.new
      end

      def type_parameters
        TypeParameterTable.new
      end

      def type_parameter_instances
        TypeParameterInstances.new
      end

      def implemented_traits
        {}
      end

      def define_attribute(name, *)
        NullSymbol.singleton
      end

      def lookup_attribute(name)
        NullSymbol.singleton
      end

      def type_compatible?(other, *)
        other = other.type if other.optional?

        if other.dynamic?
          true
        elsif other.type_parameter?
          compatible_with_type_parameter?(other)
        else
          false
        end
      end

      def compatible_with_type_parameter?(other)
        other.type_parameter? && other.required_traits.empty?
      end

      def type_name
        'Dynamic'
      end

      def resolved_return_type(*)
        self
      end
    end
  end
end
